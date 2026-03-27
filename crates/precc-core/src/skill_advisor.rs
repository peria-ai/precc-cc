//! Skill Cluster Analysis & Token-Efficient Recommendation Engine.
//!
//! Scans installed Claude Code skills (SKILL.md files), clusters them by
//! functional similarity using TF-IDF cosine similarity, monitors per-skill
//! token consumption from session logs, and recommends removing duplicates
//! to save context tokens.

use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// =============================================================================
// Skill inventory
// =============================================================================

/// An installed skill parsed from SKILL.md.
#[derive(Debug, Clone)]
pub struct InstalledSkill {
    pub name: String,
    pub description: String,
    pub path: PathBuf,
    pub content_bytes: u64,
    /// Estimated context tokens consumed when skill is loaded.
    /// Formula: (195 + 97 + name.len + description.len + body.len) / 4
    pub context_tokens: u64,
    pub source: String, // "plugin", "local", "clawhub"
}

/// Scan all installed SKILL.md files from known locations.
pub fn scan_installed_skills() -> Result<Vec<InstalledSkill>> {
    let home = std::env::var("HOME").map_err(|_| anyhow::anyhow!("HOME not set"))?;
    let mut skills = Vec::new();

    // Scan ~/.claude/plugins/**/SKILL.md
    let plugins_dir = PathBuf::from(&home).join(".claude/plugins");
    if plugins_dir.exists() {
        collect_skills_recursive(&plugins_dir, "plugin", &mut skills);
    }

    // Scan ~/.claude/skills/*/SKILL.md (ClawHub-installed)
    let skills_dir = PathBuf::from(&home).join(".claude/skills");
    if skills_dir.exists() {
        collect_skills_recursive(&skills_dir, "clawhub", &mut skills);
    }

    // Scan project-local skills/*/SKILL.md
    if let Ok(cwd) = std::env::current_dir() {
        let local_skills = cwd.join("skills");
        if local_skills.exists() {
            collect_skills_recursive(&local_skills, "local", &mut skills);
        }
    }

    Ok(skills)
}

fn collect_skills_recursive(dir: &Path, source: &str, out: &mut Vec<InstalledSkill>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // Check for SKILL.md in this directory
            let skill_md = path.join("SKILL.md");
            if skill_md.exists() {
                if let Some(skill) = parse_skill_md(&skill_md, source) {
                    out.push(skill);
                }
            }
            // Recurse
            collect_skills_recursive(&path, source, out);
        }
    }
}

fn parse_skill_md(path: &Path, source: &str) -> Option<InstalledSkill> {
    let content = std::fs::read_to_string(path).ok()?;
    let content_bytes = content.len() as u64;

    // Parse YAML frontmatter (between --- delimiters)
    let mut name = String::new();
    let mut description = String::new();

    if let Some(rest) = content.strip_prefix("---") {
        if let Some(end) = rest.find("---") {
            let frontmatter = &rest[..end];
            for line in frontmatter.lines() {
                let line = line.trim();
                if let Some(val) = line.strip_prefix("name:") {
                    name = val.trim().trim_matches('"').trim_matches('\'').to_string();
                } else if let Some(val) = line.strip_prefix("description:") {
                    description = val.trim().trim_matches('"').trim_matches('\'').to_string();
                }
            }
        }
    }

    // Fallback: use directory name if no name in frontmatter
    if name.is_empty() {
        name = path.parent()?.file_name()?.to_string_lossy().to_string();
    }

    // Token cost: (195 base + 97 per skill + field lengths) / 4
    let context_tokens = (195 + 97 + name.len() as u64 + description.len() as u64
        + content_bytes.min(4000)) // Cap body contribution
        / 4;

    Some(InstalledSkill {
        name,
        description,
        path: path.to_path_buf(),
        content_bytes,
        context_tokens,
        source: source.to_string(),
    })
}

// =============================================================================
// TF-IDF clustering
// =============================================================================

const STOPWORDS: &[&str] = &[
    "the", "a", "an", "for", "with", "and", "or", "to", "in", "of", "is", "it", "that", "this",
    "be", "are", "was", "were", "been", "has", "have", "had", "do", "does", "did", "will", "can",
    "should", "would", "could", "may", "might", "shall", "on", "at", "by", "from", "as", "into",
    "through", "during", "before", "after", "above", "below", "between", "about", "not", "no",
    "all", "each", "every", "both", "few", "more", "most", "other", "some", "such", "than", "too",
    "very", "just", "but", "if", "when", "where", "how", "what", "which", "who", "whom", "why",
    "so", "up", "out", "then", "here", "there", "these", "those", "its", "your", "my", "our",
    "their", "his", "her", "use", "used", "using", "skill", "skills", "code", "claude", "tool",
    "tools", "command", "commands", "run", "running",
];

/// Tokenize text into lowercase words, removing stopwords and short tokens.
fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '-')
        .filter(|w| w.len() > 2)
        .filter(|w| !STOPWORDS.contains(w))
        .map(|w| w.to_string())
        .collect()
}

/// Build TF-IDF vectors for a set of documents.
fn build_tfidf(docs: &[Vec<String>]) -> (Vec<HashMap<String, f64>>, HashMap<String, f64>) {
    let n = docs.len() as f64;

    // Document frequency per term
    let mut df: HashMap<String, f64> = HashMap::new();
    for doc in docs {
        let unique: std::collections::HashSet<&String> = doc.iter().collect();
        for term in unique {
            *df.entry(term.clone()).or_default() += 1.0;
        }
    }

    // IDF
    let idf: HashMap<String, f64> = df
        .iter()
        .map(|(term, count)| (term.clone(), (n / count).ln()))
        .collect();

    // TF-IDF per document
    let tfidf_vecs: Vec<HashMap<String, f64>> = docs
        .iter()
        .map(|doc| {
            let mut tf: HashMap<String, f64> = HashMap::new();
            for term in doc {
                *tf.entry(term.clone()).or_default() += 1.0;
            }
            let doc_len = doc.len() as f64;
            tf.iter()
                .map(|(term, count)| {
                    let tf_val = count / doc_len.max(1.0);
                    let idf_val = idf.get(term).copied().unwrap_or(0.0);
                    (term.clone(), tf_val * idf_val)
                })
                .collect()
        })
        .collect();

    (tfidf_vecs, idf)
}

/// Cosine similarity between two TF-IDF vectors.
fn cosine_similarity(a: &HashMap<String, f64>, b: &HashMap<String, f64>) -> f64 {
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for (term, val) in a {
        norm_a += val * val;
        if let Some(bval) = b.get(term) {
            dot += val * bval;
        }
    }
    for val in b.values() {
        norm_b += val * val;
    }

    let denom = norm_a.sqrt() * norm_b.sqrt();
    if denom < 1e-10 {
        0.0
    } else {
        dot / denom
    }
}

/// A cluster of functionally similar skills.
#[derive(Debug, Clone)]
pub struct SkillCluster {
    pub label: String,
    pub skills: Vec<String>,
    pub total_context_tokens: u64,
}

/// Cluster skills by functional similarity using TF-IDF on name + description.
/// Returns clusters with 2+ skills (singletons are omitted).
pub fn cluster_skills(skills: &[InstalledSkill], threshold: f64) -> Vec<SkillCluster> {
    if skills.len() < 2 {
        return Vec::new();
    }

    // Build documents from name + description
    let docs: Vec<Vec<String>> = skills
        .iter()
        .map(|s| {
            let text = format!("{} {}", s.name.replace('-', " "), s.description);
            tokenize(&text)
        })
        .collect();

    let (tfidf_vecs, _) = build_tfidf(&docs);

    // Simple agglomerative clustering: union-find approach
    let n = skills.len();
    let mut parent: Vec<usize> = (0..n).collect();

    fn find(parent: &mut [usize], i: usize) -> usize {
        if parent[i] != i {
            parent[i] = find(parent, parent[i]);
        }
        parent[i]
    }

    fn union(parent: &mut [usize], i: usize, j: usize) {
        let pi = find(parent, i);
        let pj = find(parent, j);
        if pi != pj {
            parent[pi] = pj;
        }
    }

    // Merge skills with similarity above threshold
    for i in 0..n {
        for j in (i + 1)..n {
            let sim = cosine_similarity(&tfidf_vecs[i], &tfidf_vecs[j]);
            if sim > threshold {
                union(&mut parent, i, j);
            }
        }
    }

    // Group by cluster root
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let root = find(&mut parent, i);
        groups.entry(root).or_default().push(i);
    }

    // Build clusters (only groups with 2+ members)
    let mut clusters: Vec<SkillCluster> = groups
        .into_values()
        .filter(|members| members.len() >= 2)
        .map(|members| {
            // Label: most frequent non-stopword across member descriptions
            let mut word_freq: HashMap<String, usize> = HashMap::new();
            for &idx in &members {
                for word in &docs[idx] {
                    *word_freq.entry(word.clone()).or_default() += 1;
                }
            }
            let mut sorted: Vec<_> = word_freq.into_iter().collect();
            sorted.sort_by(|a, b| b.1.cmp(&a.1));
            let top_words: Vec<String> = sorted.iter().take(3).map(|(w, _)| w.clone()).collect();
            let label = if top_words.is_empty() {
                "Miscellaneous".to_string()
            } else {
                top_words.join(" / ")
            };

            let skill_names: Vec<String> =
                members.iter().map(|&i| skills[i].name.clone()).collect();
            let total_tokens: u64 = members.iter().map(|&i| skills[i].context_tokens).sum();

            SkillCluster {
                label,
                skills: skill_names,
                total_context_tokens: total_tokens,
            }
        })
        .collect();

    clusters.sort_by(|a, b| b.total_context_tokens.cmp(&a.total_context_tokens));
    clusters
}

// =============================================================================
// Usage monitoring from session logs
// =============================================================================

/// Per-skill usage stats from session log analysis.
#[derive(Debug, Clone, Default)]
pub struct SkillUsage {
    pub activations: u64,
    pub total_output_tokens: u64,
    pub success_count: u64,
    pub failure_count: u64,
}

impl SkillUsage {
    pub fn avg_output_tokens(&self) -> u64 {
        if self.activations == 0 {
            0
        } else {
            self.total_output_tokens / self.activations
        }
    }

    pub fn success_rate(&self) -> f64 {
        let total = self.success_count + self.failure_count;
        if total == 0 {
            0.0
        } else {
            self.success_count as f64 / total as f64
        }
    }
}

/// Scan session logs to build per-skill usage stats.
///
/// Detects skill activations by matching tool_use names against installed skill names,
/// and by parsing PRECC reason strings for skill attribution.
pub fn scan_skill_usage(skill_names: &[String]) -> Result<HashMap<String, SkillUsage>> {
    let files = crate::mining::find_session_files()?;
    let mut usage: HashMap<String, SkillUsage> = HashMap::new();

    // Build a quick lookup set
    let name_set: std::collections::HashSet<&str> =
        skill_names.iter().map(|s| s.as_str()).collect();

    for path in &files {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        scan_session_for_skills(&content, &name_set, &mut usage);
    }

    Ok(usage)
}

fn scan_session_for_skills(
    content: &str,
    skill_names: &std::collections::HashSet<&str>,
    usage: &mut HashMap<String, SkillUsage>,
) {
    // Track pending tool_use calls: id -> (tool_name, is_skill)
    let mut pending: HashMap<String, (String, bool)> = HashMap::new();

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parsed: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let msg = match parsed.get("message") {
            Some(m) if !m.is_null() => m,
            _ => continue,
        };

        let content_val = match msg.get("content") {
            Some(c) if c.is_array() => c,
            _ => continue,
        };

        let blocks = match content_val.as_array() {
            Some(arr) => arr,
            None => continue,
        };

        for block in blocks {
            let btype = match block.get("type").and_then(|t| t.as_str()) {
                Some(t) => t,
                None => continue,
            };

            match btype {
                "tool_use" => {
                    let tool_name = block
                        .get("name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("")
                        .to_string();

                    // Check if this is a skill invocation (Skill tool with skill arg)
                    let is_skill = if tool_name == "Skill" {
                        if let Some(skill_name) = block
                            .get("input")
                            .and_then(|i| i.get("skill"))
                            .and_then(|s| s.as_str())
                        {
                            if skill_names.contains(skill_name) {
                                let entry = usage.entry(skill_name.to_string()).or_default();
                                entry.activations += 1;
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else if skill_names.contains(tool_name.as_str()) {
                        // Direct tool name matches a skill
                        let entry = usage.entry(tool_name.clone()).or_default();
                        entry.activations += 1;
                        true
                    } else {
                        false
                    };

                    if let Some(id) = block.get("id").and_then(|i| i.as_str()) {
                        pending.insert(id.to_string(), (tool_name, is_skill));
                    }
                }
                "tool_result" => {
                    let entry = block
                        .get("tool_use_id")
                        .and_then(|i| i.as_str())
                        .and_then(|id| pending.remove(id));

                    if let Some((tool_name, true)) = entry {
                        let output_len =
                            crate::nushell::estimate_tokens(&extract_tool_result_text(block))
                                as u64;

                        let is_error = block
                            .get("is_error")
                            .and_then(|e| e.as_bool())
                            .unwrap_or(false);

                        // Find the skill name (may be the tool name or from Skill input)
                        let skill_key = if tool_name == "Skill" {
                            // Already recorded in tool_use phase; find most recent
                            continue;
                        } else {
                            tool_name
                        };

                        let skill_usage = usage.entry(skill_key).or_default();
                        skill_usage.total_output_tokens += output_len;
                        if is_error {
                            skill_usage.failure_count += 1;
                        } else {
                            skill_usage.success_count += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn extract_tool_result_text(block: &serde_json::Value) -> String {
    match block.get("content") {
        Some(c) if c.is_string() => c.as_str().unwrap_or("").to_string(),
        Some(c) if c.is_array() => c
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|item| item.get("text").and_then(|t| t.as_str()))
            .collect::<Vec<_>>()
            .join(""),
        _ => String::new(),
    }
}

// =============================================================================
// Recommendation engine
// =============================================================================

/// A recommendation to remove a skill in favor of a cheaper alternative.
#[derive(Debug, Clone)]
pub struct SkillRecommendation {
    pub cluster_label: String,
    pub keep: SkillScore,
    pub remove: Vec<SkillScore>,
    pub tokens_saved_per_session: u64,
}

/// Score for a single skill within a cluster.
#[derive(Debug, Clone)]
pub struct SkillScore {
    pub name: String,
    pub context_tokens: u64,
    pub activations: u64,
    pub success_rate: f64,
    pub total_cost: u64, // context_tokens + avg_output_tokens
}

/// Generate recommendations for each cluster.
pub fn recommend(
    skills: &[InstalledSkill],
    clusters: &[SkillCluster],
    usage: &HashMap<String, SkillUsage>,
) -> Vec<SkillRecommendation> {
    let skill_map: HashMap<&str, &InstalledSkill> =
        skills.iter().map(|s| (s.name.as_str(), s)).collect();

    let mut recommendations = Vec::new();

    for cluster in clusters {
        if cluster.skills.len() < 2 {
            continue;
        }

        // Score each skill in the cluster
        let mut scores: Vec<SkillScore> = cluster
            .skills
            .iter()
            .filter_map(|name| {
                let skill = skill_map.get(name.as_str())?;
                let u = usage.get(name.as_str());
                let activations = u.map(|u| u.activations).unwrap_or(0);
                let success_rate = u.map(|u| u.success_rate()).unwrap_or(0.0);
                let avg_output = u.map(|u| u.avg_output_tokens()).unwrap_or(0);
                let total_cost = skill.context_tokens + avg_output;

                Some(SkillScore {
                    name: name.clone(),
                    context_tokens: skill.context_tokens,
                    activations,
                    success_rate,
                    total_cost,
                })
            })
            .collect();

        if scores.len() < 2 {
            continue;
        }

        // Sort: prefer most activated with good success rate, then lowest cost
        scores.sort_by(|a, b| {
            // Primary: higher activations is better (more used = more valuable)
            let act_cmp = b.activations.cmp(&a.activations);
            if act_cmp != std::cmp::Ordering::Equal {
                return act_cmp;
            }
            // Secondary: lower total cost is better
            a.total_cost.cmp(&b.total_cost)
        });

        let keep = scores.remove(0);
        let tokens_saved: u64 = scores.iter().map(|s| s.context_tokens).sum();

        recommendations.push(SkillRecommendation {
            cluster_label: cluster.label.clone(),
            keep,
            remove: scores,
            tokens_saved_per_session: tokens_saved,
        });
    }

    recommendations.sort_by(|a, b| b.tokens_saved_per_session.cmp(&a.tokens_saved_per_session));
    recommendations
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_basic() {
        let tokens = tokenize("Hello World foo-bar baz");
        assert!(tokens.contains(&"hello".to_string()));
        assert!(tokens.contains(&"world".to_string()));
        assert!(tokens.contains(&"foo-bar".to_string()));
        assert!(tokens.contains(&"baz".to_string()));
    }

    #[test]
    fn tokenize_removes_stopwords() {
        let tokens = tokenize("the quick brown fox and the lazy dog");
        assert!(!tokens.contains(&"the".to_string()));
        assert!(!tokens.contains(&"and".to_string()));
        assert!(tokens.contains(&"quick".to_string()));
        assert!(tokens.contains(&"brown".to_string()));
    }

    #[test]
    fn tokenize_removes_short() {
        let tokens = tokenize("a an to go is");
        assert!(tokens.is_empty()); // all are stopwords or <= 2 chars
    }

    #[test]
    fn cosine_similarity_identical() {
        let mut a = HashMap::new();
        a.insert("test".to_string(), 1.0);
        a.insert("code".to_string(), 0.5);
        let sim = cosine_similarity(&a, &a);
        assert!((sim - 1.0).abs() < 0.01);
    }

    #[test]
    fn cosine_similarity_orthogonal() {
        let mut a = HashMap::new();
        a.insert("test".to_string(), 1.0);
        let mut b = HashMap::new();
        b.insert("deploy".to_string(), 1.0);
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 0.01);
    }

    #[test]
    fn cluster_identical_descriptions() {
        let skills = vec![
            InstalledSkill {
                name: "review-pr".to_string(),
                description: "Review pull requests and provide feedback".to_string(),
                path: PathBuf::from("/tmp/a"),
                content_bytes: 500,
                context_tokens: 200,
                source: "plugin".to_string(),
            },
            InstalledSkill {
                name: "pr-reviewer".to_string(),
                description: "Review pull requests and give feedback on code".to_string(),
                path: PathBuf::from("/tmp/b"),
                content_bytes: 600,
                context_tokens: 220,
                source: "clawhub".to_string(),
            },
            InstalledSkill {
                name: "deploy-app".to_string(),
                description: "Deploy application to cloud infrastructure".to_string(),
                path: PathBuf::from("/tmp/c"),
                content_bytes: 400,
                context_tokens: 180,
                source: "local".to_string(),
            },
        ];

        let clusters = cluster_skills(&skills, 0.3);
        // review-pr and pr-reviewer should cluster together
        assert!(
            clusters.iter().any(|c| c.skills.len() == 2),
            "Expected one cluster with 2 skills, got: {:?}",
            clusters
        );
        // deploy-app should not be in any cluster (singleton)
        let all_clustered: Vec<&str> = clusters
            .iter()
            .flat_map(|c| c.skills.iter().map(|s| s.as_str()))
            .collect();
        assert!(!all_clustered.contains(&"deploy-app"));
    }

    #[test]
    fn cluster_no_overlap() {
        let skills = vec![
            InstalledSkill {
                name: "git-helper".to_string(),
                description: "Git workflow automation".to_string(),
                path: PathBuf::from("/tmp/a"),
                content_bytes: 300,
                context_tokens: 150,
                source: "plugin".to_string(),
            },
            InstalledSkill {
                name: "deploy-aws".to_string(),
                description: "Deploy to AWS infrastructure".to_string(),
                path: PathBuf::from("/tmp/b"),
                content_bytes: 400,
                context_tokens: 170,
                source: "clawhub".to_string(),
            },
        ];

        let clusters = cluster_skills(&skills, 0.5);
        assert!(clusters.is_empty(), "Unrelated skills should not cluster");
    }

    #[test]
    fn recommend_picks_most_used() {
        let skills = vec![
            InstalledSkill {
                name: "review-a".to_string(),
                description: "Code review".to_string(),
                path: PathBuf::from("/tmp/a"),
                content_bytes: 500,
                context_tokens: 200,
                source: "plugin".to_string(),
            },
            InstalledSkill {
                name: "review-b".to_string(),
                description: "Code review".to_string(),
                path: PathBuf::from("/tmp/b"),
                content_bytes: 300,
                context_tokens: 150,
                source: "clawhub".to_string(),
            },
        ];

        let clusters = vec![SkillCluster {
            label: "review".to_string(),
            skills: vec!["review-a".to_string(), "review-b".to_string()],
            total_context_tokens: 350,
        }];

        let mut usage = HashMap::new();
        usage.insert(
            "review-a".to_string(),
            SkillUsage {
                activations: 20,
                total_output_tokens: 1000,
                success_count: 18,
                failure_count: 2,
            },
        );
        usage.insert(
            "review-b".to_string(),
            SkillUsage {
                activations: 3,
                total_output_tokens: 300,
                success_count: 2,
                failure_count: 1,
            },
        );

        let recs = recommend(&skills, &clusters, &usage);
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].keep.name, "review-a"); // More activations
        assert_eq!(recs[0].remove.len(), 1);
        assert_eq!(recs[0].remove[0].name, "review-b");
        assert_eq!(recs[0].tokens_saved_per_session, 150);
    }

    #[test]
    fn skill_usage_defaults() {
        let u = SkillUsage::default();
        assert_eq!(u.avg_output_tokens(), 0);
        assert_eq!(u.success_rate(), 0.0);
    }

    #[test]
    fn parse_skill_md_format() {
        let dir = tempfile::tempdir().unwrap();
        let skill_dir = dir.path().join("test-skill");
        std::fs::create_dir(&skill_dir).unwrap();
        let skill_md = skill_dir.join("SKILL.md");
        std::fs::write(
            &skill_md,
            "---\nname: test-skill\ndescription: A test skill for testing\n---\n# Test\nBody here.",
        )
        .unwrap();

        let skill = parse_skill_md(&skill_md, "local").unwrap();
        assert_eq!(skill.name, "test-skill");
        assert_eq!(skill.description, "A test skill for testing");
        assert!(skill.context_tokens > 0);
        assert_eq!(skill.source, "local");
    }
}

// live-stats.js — Fetch live PRECC stats and inject into data-stat spans.
// Loaded by mdbook via additional-js. No rebuild needed when stats update.
(function () {
  var STATS_URL = '/api/stats.json';
  var spans = document.querySelectorAll('[data-stat]');
  var versionTable = document.getElementById('version-breakdown');
  if (!spans.length && !versionTable) return;

  fetch(STATS_URL)
    .then(function (r) { return r.json(); })
    .then(function (data) {
      // Inject scalar stats into data-stat spans
      spans.forEach(function (el) {
        var key = el.getAttribute('data-stat');
        if (data[key] !== undefined) {
          var val = data[key];
          if (typeof val === 'number' && val >= 1000) {
            val = val.toLocaleString();
          }
          el.textContent = val;
          el.style.fontWeight = '600';
        }
      });

      // Inject current version label
      if (data.current_version) {
        var verSpans = document.querySelectorAll('[data-stat="current_version"]');
        verSpans.forEach(function (el) {
          el.textContent = 'v' + data.current_version;
          el.style.fontWeight = '600';
        });
      }

      // Build per-version breakdown table
      if (versionTable && data.by_version && data.by_version.length > 0) {
        var html = '';
        data.by_version.forEach(function (v) {
          var saved = typeof v.tokens_saved === 'number' ? v.tokens_saved.toLocaleString() : v.tokens_saved;
          var hooks = typeof v.total_invocations === 'number' ? v.total_invocations.toLocaleString() : v.total_invocations;
          var users = v.users || '?';
          html += '<tr>'
            + '<td><strong>v' + v.version + '</strong></td>'
            + '<td>' + users + '</td>'
            + '<td>' + hooks + '</td>'
            + '<td>' + saved + '</td>'
            + '<td><strong>' + v.saving_pct + '%</strong></td>'
            + '</tr>';
        });
        versionTable.querySelector('tbody').innerHTML = html;
        versionTable.style.display = '';
      }
    })
    .catch(function () {
      // Fail silently — fallback text remains
    });
})();

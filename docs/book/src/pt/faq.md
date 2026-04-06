# Perguntas frequentes

## O PRECC é seguro?

Sim. PRECC usa o mecanismo oficial de hooks PreToolUse do Claude Code -- o mesmo ponto de extensão que a Anthropic projetou exatamente para esse propósito. O hook:

- Roda inteiramente offline (sem chamadas de rede no caminho crítico)
- Completa em menos de 5 milissegundos
- É fail-open: se algo der errado, o comando original roda sem modificação
- Apenas modifica comandos, nunca os executa
- Armazena dados localmente em bancos SQLite

## O PRECC funciona com outras ferramentas de codificação IA?

PRECC é projetado especificamente para o Claude Code. Ele depende do protocolo de hooks PreToolUse que o Claude Code fornece. Não funciona com Cursor, Copilot, Windsurf ou outras ferramentas de codificação IA.

## Quais dados a telemetria envia?

A telemetria é apenas opt-in. Quando ativada, envia:

- Versão do PRECC, SO e arquitetura
- Contagens agregadas (comandos interceptados, habilidades ativadas)
- Latência média do hook

**Não** envia texto de comandos, caminhos de arquivos, nomes de projetos ou informações pessoais identificáveis. Você pode visualizar a carga exata com `precc telemetry preview` antes de ativar. Veja [Telemetria](telemetry.md) para detalhes.

## Como desinstalar o PRECC?

??faq_uninstall_a_intro??

1. Remover o registro do hook:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Remover o binário:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Remover dados (opcional):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Minha licença expirou. O que acontece?

PRECC volta ao nível Community. Toda a funcionalidade principal continua funcionando:

- Habilidades integradas permanecem ativas
- O pipeline do hook funciona normalmente
- `precc savings` mostra a visão resumida
- `precc ingest` e mineração de sessões funcionam

Os recursos Pro ficam indisponíveis até a renovação:

- `precc savings --all` (detalhamento completo)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Relatórios por email

## O hook não parece estar funcionando. Como depurar?

??faq_debug_a_intro??

1. Verifique se o hook está registrado:
   ```bash
   precc init
   ```

2. Teste o hook manualmente:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Verifique se o binário está no seu PATH:
   ```bash
   which precc-hook
   ```

4. Verifique a configuração do hook do Claude Code em `~/.claude/settings.json`.

## O PRECC deixa o Claude Code mais lento?

Não. O hook completa em menos de 5 milissegundos (p99). Isso é imperceptível comparado ao tempo que o Claude gasta raciocinando e gerando respostas.

## Posso usar o PRECC em CI/CD?

PRECC é projetado para sessões interativas do Claude Code. Em CI/CD, não há instância do Claude Code para conectar. No entanto, `precc gha` pode analisar execuções falhas do GitHub Actions de qualquer ambiente.

## Como as habilidades mineradas diferem das integradas?

Habilidades integradas vêm com PRECC e cobrem padrões comuns de diretório errado. Habilidades mineradas são aprendidas de seus logs de sessão específicos -- capturam padrões únicos do seu fluxo de trabalho. Ambas são armazenadas em SQLite e avaliadas identicamente pelo pipeline do hook.

## Posso compartilhar habilidades com minha equipe?

Sim. Exporte qualquer habilidade para TOML com `precc skills export NAME` e compartilhe o arquivo. Membros da equipe podem colocá-lo no diretório `skills/` ou importá-lo para o banco de dados de heurísticas.

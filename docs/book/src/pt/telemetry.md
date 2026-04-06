# Telemetria

PRECC suporta telemetria anônima opcional para ajudar a melhorar a ferramenta. Nenhum dado é coletado a menos que você consinta explicitamente.

## Ativar

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## Desativar

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## Verificar status

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## Pré-visualização do que seria enviado

Antes de ativar, você pode ver exatamente quais dados seriam coletados:

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## O que é coletado

- Versão do PRECC, SO e arquitetura
- Contagens agregadas: comandos interceptados, habilidades ativadas, pilares usados
- Latência média do hook
- Contagem de sessões

## O que NÃO é coletado

- Sem texto de comandos ou argumentos
- Sem caminhos de arquivos ou nomes de diretórios
- Sem nomes de projetos ou URLs de repositórios
- Sem informações pessoais identificáveis (PII)
- Sem endereços IP (o servidor não os registra)

## Substituição por variável de ambiente

Para desativar a telemetria sem executar um comando (útil em CI ou ambientes compartilhados):

```bash
export PRECC_NO_TELEMETRY=1
```

Isso tem precedência sobre a configuração de consentimento.

## Destino dos dados

Os dados de telemetria são enviados para `https://telemetry.peria.ai/v1/precc` via HTTPS. Os dados são usados exclusivamente para entender padrões de uso e priorizar o desenvolvimento.

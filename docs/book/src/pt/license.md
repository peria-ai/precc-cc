# Licença

PRECC oferece dois níveis: Community (gratuito) e Pro.

## Nível Community (gratuito)

O nível Community inclui:

- Todas as habilidades integradas (correção de diretório, tradução jj, etc.)
- Pipeline de hooks com suporte completo a Pillar 1 e Pillar 4
- Resumo básico de `precc savings`
- Mineração de sessões com `precc ingest`
- Uso local ilimitado

## Nível Pro

Pro desbloqueia recursos adicionais:

- **Detalhamento de economias** -- `precc savings --all` com análise por comando
- **Gravação de GIF** -- `precc gif` para criar GIFs animados de terminal
- **Conformidade de geofence IP** -- Para ambientes regulamentados
- **Relatórios por e-mail** -- `precc mail report` para enviar análises
- **Análise de GitHub Actions** -- `precc gha` para depuração de workflows falhados
- **Compressão de contexto** -- `precc compress` para otimização do CLAUDE.md
- **Suporte prioritário**

## Ativando uma licença

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Verificando o status da licença

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## Ativação por GitHub Sponsors

Se você patrocina PRECC pelo GitHub Sponsors, sua licença é ativada automaticamente via seu e-mail do GitHub. Nenhuma chave necessária -- apenas certifique-se de que seu e-mail de patrocinador corresponde:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Impressão digital do dispositivo

Cada licença está vinculada a uma impressão digital do dispositivo. Veja a sua com:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Se precisar transferir sua licença para uma nova máquina, desative primeiro:

```bash
precc license deactivate
```

Depois ative na nova máquina.

## Licença expirada?

Quando uma licença Pro expira, PRECC retorna ao nível Community. Todas as habilidades integradas e funcionalidades principais continuam funcionando. Apenas recursos específicos do Pro ficam indisponíveis. Veja o [FAQ](faq.md) para mais detalhes.

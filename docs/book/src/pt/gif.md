# Gravação GIF

`precc gif` cria gravações GIF animadas de sessões de terminal a partir de scripts bash. Este é um recurso Pro.

## Uso básico

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

O primeiro argumento é um script bash contendo os comandos a executar. O segundo argumento é a duração máxima da gravação.

## Formato do script

O script é um arquivo bash padrão:

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## Simulação de entrada

Para comandos interativos, forneça valores de entrada como argumentos adicionais:

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

Cada argumento adicional é fornecido como uma linha de stdin quando o script solicita entrada.

## Opções de saída

O arquivo de saída é nomeado com base no script por padrão (`script.gif`). O GIF usa um tema de terminal escuro com dimensões padrão 80x24.

## Por que GIF ao invés de asciinema?

A habilidade integrada `asciinema-gif` reescreve automaticamente `asciinema rec` para `precc gif`. Arquivos GIF são mais portáteis -- eles são exibidos inline em READMEs do GitHub, Slack e e-mail sem necessitar de um player.

# RPatchur

[![Rust](https://github.com/ArturllVale/rpatchur_v2/actions/workflows/rust.yml/badge.svg)](https://github.com/ArturllVale/rpatchur_v2/actions/workflows/rust.yml)

`rpatchur` é um atualizador (patcher) customizável e multiplataforma para clientes Ragnarok Online.

## Funcionalidades

* Interface de usuário (UI) baseada na web e customizável
* Configurável através de um arquivo YAML externo
* Suporte a HTTP/HTTPS
* Aplicação de patches em arquivos GRF (versões 0x101, 0x102, 0x103 e 0x200)
* Suporte ao formato de patch THOR
* Substituto direto para o Thor Patcher
* Suporte a login SSO (pode atuar como um launcher)
* Aplicação manual de patches
* Pode utilizar múltiplos espelhos (mirrors) de patch
* Multiplataforma (Windows, Linux, macOS)

## Limitações Conhecidas

* Nenhuma conhecida no momento.

## Screenshot

![screen](https://i.imgur.com/mE51Iif.png)

## Documentação

Você pode encontrar a documentação original do projeto [aqui](https://l1nkz.github.io/rpatchur/).

## Guia para Iniciantes

Se você não tem muita experiência com programação ou com a configuração de servidores de Ragnarok, siga este guia básico para começar.

### O que o RPatchur faz?
Ele verifica um arquivo de lista de atualizações (geralmente chamado `plist.txt`) hospedado no seu site, baixa os arquivos necessários e os aplica ao jogo do usuário.

### Passo 1: Configuração do Patcher (`rpatchur.yml`)
O arquivo `rpatchur.yml` é onde você diz ao patcher onde buscar as atualizações e qual executável do jogo abrir. Ele deve ficar na mesma pasta do executável do `rpatchur`.

Exemplo básico de configuração:

```yaml
window:
  title: Meu Servidor   # Título da janela
  width: 780
  height: 580
  # Funcionalidades extras:
  frameless: true                 # Oculta bordas e barra de título
  transparent_color_hex: "FF00FF" # Define cor para transparência (janelas com formatos customizados)

# Comandos JavaScript para UI (janelas sem borda)
# minimize   -> external.invoke('minimize')
# start_drag -> external.invoke('start_drag') (chamar no mousedown)

play:
  path: ragexe.exe      # Nome do executável do seu jogo
  arguments: ["1sak1"]  # Argumentos (se necessário)

web:
  # Página HTML que será exibida dentro do patcher (suas notícias, banner, etc)
  index_url: https://meuservidor.com/patcher/index.html
  
  # Configuração de onde baixar os arquivos
  patch_servers:
    - name: Servidor Principal
      # Arquivo de texto contendo a lista de patches (ex: 1.thor, 2.thor)
      plist_url: https://meuservidor.com/patcher/plist.txt
      # Pasta onde estão os arquivos .thor ou .grf
      patch_url: https://meuservidor.com/patcher/data/

client:
  default_grf_name: meuservidor.grf  # Nome do seu GRF principal
```

### Passo 2: Criando Patches
Para atualizar o jogo dos jogadores, você precisa criar arquivos de patch. Este projeto inclui uma ferramenta chamada `mkpatch` para isso.

1. Crie um arquivo `patch.yml` definindo o que vai mudar (veja `examples/patch.yml` para inspiração).
2. Use o utilitário `mkpatch` (que você precisa compilar ou baixar) para gerar o arquivo `.thor`.

Exemplo de `patch.yml`:
```yaml
use_grf_merging: true          # true para colocar arquivos dentro do GRF
target_grf_name: meuservidor.grf

entries:
  - relative_path: data\clientinfo.xml # Arquivo local para adicionar
    in_grf_path: data\clientinfo.xml   # Caminho dentro do GRF
```

### Passo 3: Hospedando os Arquivos
No seu servidor web (hospedagem de site), você precisa ter uma estrutura assim:

* `index.html`: A página bonitinha que aparece no patcher.
* `plist.txt`: Uma lista simples com os números/nomes dos patches.
* `data/`: Uma pasta com os arquivos `.thor` gerados.

Exemplo de `plist.txt`:
```
1.thor
2.thor
updates_jan.thor
```

#### Formatos Suportados
O rpatchur foi projetado para trabalhar primariamente com **atualizações de GRF** (método moderno e mais utilizado), mas mantém compatibilidade com formatos antigos.

* **.rgz** e **.gpf** (Principal): Arquivos GRF comprimidos (Gzip). Este é o método recomendado. O rpatchur irá descomprimir e mesclar o conteúdo destes arquivos diretamente no GRF principal do seu servidor.
* **.thor** (Secundário): Formato legado do Thor Patcher. Suportado para retrocompatibilidade.
* **.grf**: Para usar arquivos .grf puros, é necessário comprimi-los como `.rgz` ou `.gpf` para que o patcher possa realizar o merge.


## Exemplos

Você pode encontrar arquivos de exemplo para a interface e configuração na pasta `examples`.

## Compilação (Building)

O diretório `rpatchur` contém o código do patcher (UI, fusão de arquivos, etc).
O diretório `mkpatch` contém o utilitário de geração de patches THOR.
O diretório `gruf` contém a biblioteca principal para leitura e escrita de arquivos GRF e THOR.

Para clonar o repositório e compilar tudo, execute:
```bash
$ git clone https://github.com/ArturllVale/rpatchur_v2.git
$ cd rpatchur_v2
$ cargo build --release
```

Nota: Rust 1.49 ou superior é necessário.

Para compilar para Windows 32-bit em um sistema 64-bit:
```bash
$ rustup target add i686-pc-windows-msvc
$ cargo build --target=i686-pc-windows-msvc --release
```

## Após a Compilação

Após o comando de build terminar com sucesso, você encontrará o executável na pasta `target/release`. 

Para o patcher funcionar, você precisa criar uma pasta separada (onde você quiser) e organizar a estrutura da seguinte forma:

1. Copie o executável (ex: `rpatchur.exe`) da pasta `target/release` para sua nova pasta.
2. Crie ou copie o arquivo `rpatchur.yml` para a mesma pasta do executável.
3. Certifique-se de que as configurações no `rpatchur.yml` apontam para os lugares certos (seu site ou arquivos locais).

## Como Editar o Estilo (UI)

O visual do rpatchur é feito inteiramente com **HTML e CSS**, igual a um site.

Para editar o estilo:

1. Crie um arquivo HTML (ex: `index.html`) e seus arquivos CSS/imagens. Você pode usar os exemplos na pasta `examples` deste repositório como base.
2. Edite o `rpatchur.yml` para apontar para este arquivo.
   
   Para testar localmente (sem precisar subir num site):
   ```yaml
   web:
     index_url: file:///C:/Caminho/Para/Seu/index.html
   ```

3. Abra o `rpatchur.exe`. Ele vai carregar o seu HTML.
4. Edite o HTML/CSS e reabra o patcher para ver as mudanças.

**Dica:** Você pode criar botões que interagem com o patcher (como "Jogar", "Sair") usando os IDs específicos no HTML. Veja os arquivos na pasta `examples/basic_launcher` para ver como os botões `start-btn` e `exit-btn` funcionam.

### Compilação Cruzada (Cross Compilation)

Recomenda-se compilar na plataforma de destino. No entanto, há um `Dockerfile` na pasta `docker` para facilitar a compilação do Linux para Windows.

## Licença

Copyright (c) 2020-2026 desenvolvedores rpatchur e mantenedor: Lumen#0110


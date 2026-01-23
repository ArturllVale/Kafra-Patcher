# Guia de Uso do MKPatch

O `mkpatch.exe` é uma ferramenta de linha de comando (CLI) e **não possui interface gráfica**. Se você tentar abri-lo dando dois cliques, ele abrirá e fechará rapidamente, pois espera receber argumentos.

## Como Usar

Para usar o `mkpatch`, você deve executá-lo através do **Prompt de Comando (CMD)** ou **PowerShell**.

### Sintaxe Básica

```bash
mkpatch.exe <caminho-para-o-arquivo-de-definicao>
```

### Passo a Passo

1.  **Prepare o Arquivo de Definição**:
    Crie um arquivo `.yml` (ex: `patch.yml`) descrevendo o que o patch deve fazer.
    
    Exemplo (`patch.yml`):
    ```yaml
    use_grf_merging: true
    target_grf_name: seuserver.grf
    entries:
      - relative_path: data\clientinfo.xml
    ```

2.  **Abra o Terminal**:
    Navegue até a pasta onde está o `mkpatch.exe`.

3.  **Execute o Comando**:
    ```bash
    .\mkpatch.exe patch.yml
    ```

4.  **Resultado**:
    Se tudo correr bem, um arquivo `.thor` será gerado na mesma pasta (ou onde especificado).

## Solução de Problemas

*   **"O programa fecha sozinho"**: Isso é normal se você não passar argumentos. Use o terminal.
*   **Erro "patch definition file not provided"**: Você esqueceu de indicar o arquivo `.yml`.

## Exemplo Completo

Suponha que você tenha:
*   `mkpatch.exe`
*   `patch.yml`
*   Pasta `data/` com seus arquivos modificados.

Comando:
```bash
.\mkpatch.exe patch.yml
```

Isso criará `patch.thor`.

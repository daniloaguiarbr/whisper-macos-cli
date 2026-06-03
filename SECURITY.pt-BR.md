[English version](SECURITY.md) | [Versão em Português Brasileiro](SECURITY.pt-BR.md)

# Política de Segurança

## Versões Suportadas

| Versão | Suportada         |
|--------|-------------------|
| 0.1.x  | Sim               |
| < 0.1  | Não               |

## Reportando uma Vulnerabilidade

Reporte vulnerabilidades via GitHub Security Advisories em
https://github.com/daniloaguiarbr/whisper-macos-cli/security/advisories/new

NÃO abra issue pública para vulnerabilidades.

### O que Incluir

- Descrição da vulnerabilidade e seu impacto
- Passos para reproduzir ou prova de conceito
- Versões afetadas
- Sua avaliação de severidade (baixa, média, alta, crítica)
- Mitigações conhecidas

### Tempo de Resposta

- Triagem inicial: dentro de 72 horas
- Atualização de status: dentro de 7 dias
- Patch: dentro de 30 dias para vulnerabilidades altas ou críticas

### Divulgação Coordenada

Seguimos divulgação coordenada. Solicitamos que você não divulgue
publicamente até que tenhamos lançado correção ou 90 dias tenham
se passado, o que vier primeiro.

## Política de Divulgação

Vulnerabilidades confirmadas são publicadas no GitHub Security
Advisories após o lançamento do patch.

## Política de Atualização de Segurança

Patches de segurança são lançados em até 7 dias da confirmação
para severidades altas e críticas. Patches para severidades
menores seguem o ciclo regular de release.

## Garantias de Segurança

- Todos os downloads de modelo são verificados via hash SHA256
- Comunicação exclusivamente HTTPS com Hugging Face
- Validação de certificado TLS habilitada por padrão
- Sem telemetria ou comportamento de phone-home
- Todos os dados de áudio são processados localmente na máquina
  do usuário
- Saída de transcrição não é transmitida para nenhum serviço
  externo

## Modelo de Ameaça

A CLI é projetada para rodar como processo local de usuário em
macOS Apple Silicon. NÃO é projetada para:

- Ser exposta como serviço de rede
- Processar arquivos de modelo não confiáveis (verifique hashes
  antes de carregar)
- Rodar como root ou com privilégios elevados
- Operar em ambiente multi-tenant

## Isolamento do Subprocesso ffmpeg (v0.1.2+)

Desde a v0.1.2, a CLI pode invocar `ffmpeg` como subprocesso para
extrair áudio de containers de vídeo e como fallback quando o
decode nativo de OGG/Opus falha. O subprocesso é invocado com
as seguintes garantias de endurecimento:

- env_clear: o processo filho não herda nenhuma variável de
  ambiente do pai. Apenas uma allowlist explícita de `PATH`,
  `HOME`, `TMPDIR`, `LANG`, `LC_ALL` é readicionada. Isso
  previne vazamento acidental de segredos via logs de erro do
  ffmpeg.
- setsid (Unix) / CREATE_NEW_PROCESS_GROUP (Windows): o filho
  roda em seu próprio grupo de processos. SIGINT entregue à
  CLI pai não propaga silenciosamente para o ffmpeg, permitindo
  que o pai faça shutdown gracioso enquanto deixa o filho ter
  seu próprio ciclo de vida.
- Kill-on-drop: o handle do filho é envolvido em um guard
  SafeChild com implementação de Drop que envia SIGKILL (Unix)
  ou TerminateProcess (Windows) em panic do pai. Isso previne
  processos zumbi do ffmpeg.
- Timeout limitado: timeout padrão de 180s por invocação. No
  timeout, o filho é morto e `Error::VideoExtractionFailed` é
  retornado.
- Validação de saída WAV: o WAV extraído é validado pós-processo.
  O header deve ser `RIFF...WAVE` e o chunk size deve bater com
  o tamanho do arquivo menos 8. Isso captura a classe de bugs
  onde ffmpeg sai com 0 mas produz arquivo vazio ou truncado.
- Cleanup de temp: o arquivo WAV temporário é removido via
  guard Drop mesmo se o decode entrar em panic ou o processo
  for interrompido.
- Validação de magic bytes ANTES da invocação do ffmpeg: o
  arquivo de input é examinado em busca de magic bytes de
  container de vídeo antes de o ffmpeg ser invocado. Isso se
  recusa a invocar o ffmpeg em arquivos não-vídeo renomeados.

O subprocesso é invocado via `std::process::Command` com
`env_clear()` e `pre_exec` (Unix) ou `creation_flags` (Windows).
O filho NÃO é linkado no binário. O ffmpeg deve ser instalado
separadamente. Se não estiver, a CLI retorna código de saída 69
com hint de instalação claro.

## Limitações Conhecidas

- O modelo Whisper de 3GB é carregado inteiramente em memória
  unificada
- VAD (Voice Activity Detection) pode falhar em fala quieta
- Deduplicação baseada em hash não é realizada entre invocações
- Áudio é mantido em memória durante transcrição; arquivos grandes
  aumentam o pico de consumo de memória
- ffmpeg é um binário externo, não empacotado. O comportamento
  depende da versão do ffmpeg instalada pelo usuário
- Arquivos WAV temporários são escritos no diretório temp do
  sistema; usuários com temp restrito podem sobrescrever via
  variável de ambiente `TMPDIR`

## Criptografia

Este projeto usa primitivas criptográficas padrão do ecossistema
Rust:

- `sha2` para verificação de integridade de modelo
- `rustls` para conexões TLS
- `uuid` v7 para identificadores de correlação

Nenhum código criptográfico customizado está incluído no projeto.

## Hall of Fame

Nenhuma vulnerabilidade reportada até o momento.

## Melhores Práticas para Usuários

- Sempre baixe binários do GitHub Releases oficial
- Verifique o SHA256 contra o arquivo SHA256SUMS antes de instalar
- Use Trusted Publishing via OIDC ao publicar de CI
- Mantenha a versão do whisper-macos-cli atualizada
- Reporte comportamento suspeito via canais privados

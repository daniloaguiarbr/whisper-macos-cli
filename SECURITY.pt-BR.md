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

## Hall of Fame

Nenhuma vulnerabilidade reportada até o momento.

## Melhores Práticas para Usuários

- Sempre baixe binários do GitHub Releases oficial
- Verifique o SHA256 contra o arquivo SHA256SUMS antes de instalar
- Use Trusted Publishing via OIDC ao publicar de CI
- Mantenha a versão do whisper-macos-cli atualizada
- Reporte comportamento suspeito via canais privados

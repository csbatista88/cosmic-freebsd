# /home/christian/ports/cosmic-edit/Makefile

PORTNAME=       cosmic-edit
DISTVERSION=    1.0.0-beta.4
CATEGORIES=     x11 editors

# ... (outras variáveis como MAINTAINER, COMMENT, etc.) ...

# =========================================================================
# Referência Local da Dependência (libcosmic)
# =========================================================================

# 1. Define o diretório pai (que é /home/christian/ports/) como a MASTERDIR.
# O .CURDIR é /home/christian/ports/cosmic-edit
# O diretório pai é, portanto, ${.CURDIR}/..
MASTERDIR=      ${.CURDIR}/..

# 2. Define a dependência de build (compilação) para o libcosmic.
# O sistema de Ports sabe que deve procurar libcosmic no diretório irmão 'libcosmic'.
# A PORTVERSION deve ser a mesma no Makefile de libcosmic.
BUILD_DEPENDS+= ${PORTNAME:S/edit/cosmic/}-${DISTVERSION}:${MASTERDIR}/libcosmic

# Isso é um placeholder, pois estamos simulando a compilação do subdiretório.
WRKSRC=         ${GITHUB_WRKSRC}/cosmic-edit
# ... (restante das USES, CARGO_FEATURES, etc.) ...

# =========================================================================
# Tarefas de Compilação
# =========================================================================

do-build:
	# O comando do cargo precisará ser corrigido para usar a dependência local
	# O sistema de Ports gerencia a inclusão da biblioteca no PATH do cargo/linker
	# quando a dependência é resolvida.
	@${SETENV} ${CARGO_ENV} ${CARGO_BIN} ${CARGO_BUILD_ARGS} \
        --features ${CARGO_FEATURES} \
        --target-dir ${WRKDIR}/target \
        --release \
        --manifest-path ${WRKSRC}/Cargo.toml

# ... (do-install, etc.) ...

.include <bsd.port.mk>

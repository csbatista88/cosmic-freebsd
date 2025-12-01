# /home/christian/ports/libcosmic/Makefile
PORTNAME=	libcosmic
DISTVERSIONPREFIX=	epoch-

#remove distdir
DISTDIR=	${.CURDIR}/../distfiles

DISTVERSION=	1.0.0-beta.4
CATEGORIES=	devel graphics

MAINTAINER=	christian@FreeBSD.org
COMMENT=	Bibliotecas comuns e toolkit para o COSMIC DE (Rust)
WWW=		https://github.com/pop-os/libcosmic

BUILD_DEPENDS=	rust>=1.90.0:lang/rust

USES=		cargo xorg



USE_XORG=	pixman
USE_GITHUB=	yes
GH_ACCOUNT=	pop-os
GH_PROJECT=	libcosmic
GH_TAGNAME=	38004239


# udev nativo
CARGO_FEATURES=	 --no-default-features

WRKSRC_DIR=	${WRKDIR}/${GH_PROJECT}-${GH_TAGNAME}
#WRKSRC_SUBDIR=	libcosmic-${GH_TAGNAME}

#cargo home
#CARGO_HOME=	${WRKDIR}/.cargo

# LICENSE

LICENSE=	GPLv3+
LICENSE_FILE=	${WRKSRC_DIR}/LICENSE

# =========================================================================
# Tarefas de Compilação
# =========================================================================

GH_TUPLE=	pop-os:iced:7bb364e:iced/iced

.if 0 # breaks epoch-1.0.0-beta.1
# XXX Drop [patch.<url>] section to help "make cargo-configure"
	@${REINPLACE_CMD} -e '/\[patch/,$$d' \
		${WRKSRC}/Cargo.toml
.endif

do-build:
# Executa o cargo build com as features desabilitadas
	@${SETENV} ${CARGO_ENV} ${CARGO_BIN} ${CARGO_BUILD_ARGS} \
		--features ${CARGO_FEATURES} \
		--target-dir ${WRKSRC_DIR}/target \
		--release \
		--manifest-path ${WRKSRC_DIR}/Cargo.toml

# =========================================================================
# Tarefas de Instalação (Instala a biblioteca para linkagem)
# =========================================================================

do-install:
    # Instala o arquivo da biblioteca Rust (.rlib) para que outros Ports possam encontrá-lo
	@${MKDIR} ${STAGEDIR}${PREFIX}/lib/rustlib/${PORTNAME}
	${INSTALL_DATA} ${WRKDIR}/target/release/lib${PORTNAME}.rlib \
	${STAGEDIR}${PREFIX}/lib/rustlib/${PORTNAME}/

.include <bsd.port.mk>

# /Home/christian/ports/cosmic-comp/Makefile
PORTNAME=	cosmic-session
DISTVERSIONPREFIX=	epoch-
DISTVERSION=	1.0.0-beta.7
CATEGORIES=	x11 wayland

MAINTAINER=	christian@FreeBSD.org
COMMENT=	O gerenciador de sessão para o ambiente de desktop COSMIC (Rust)
WWW=		https://github.com/pop-os/cosmic-comp

#local distfiles
DISTDIR=	${.CURDIR}/../distfiles

LICENSE=	GPLv3+
LICENSE_FILE=	${WRKSRC}/LICENSE


LIB_DEPENDS=	libudev.so:devel/libudev-devd \
		libseat.so:sysutils/seatd


BUILD_DEPENDS=	rust>=1.90.0:lang/rust

# =========================================================================
# Configuração do Cargo e GitHub
# =========================================================================

USES=		cargo
USE_GITHUB=	yes

GH_ACCOUNT=	pop-os
GH_PROJECT=	cosmic-session
GH_TAGNAME=	epoch-1.0.0-beta.7

CARGO_FEATURES+=--no-default-features

CARGO_VENDOR_DIR=	${WRKSRC}/cargo-crates

# =========================================================================
# Arquivos Instalados (PLIST)
# =========================================================================

PLIST_FILES=	bin/${PORTNAME} \
		share/wayland-sessions/cosmic.desktop

# =========================================================================
# Tasks
# =========================================================================


post-extract:
	@${ECHO_MSG} "=> Renaming LICENSE.md to LICENSE for dbus-settings-bindings"
	@if [ -f ${WRKSRC}/LICENSE.md ]; then \
		${MV} ${WRKSRC}/LICENSE.md ${WRKSRC}/LICENSE; \
	fi


do-install:
	# Instala o binário principal
	${INSTALL_PROGRAM} ${WRKDIR}/target/release/${PORTNAME} \
		${STAGEDIR}${PREFIX}/bin/

	# Instala o arquivo de sessão .desktop (Caminho e Nome Corretos)
	${INSTALL_DATA} ${WRKSRC}/data/cosmic.desktop \
		${STAGEDIR}${PREFIX}/share/wayland-sessions/cosmic.desktop

.include <bsd.port.mk>

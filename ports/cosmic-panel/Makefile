# /Home/christian/ports/cosmic-panel/Makefile
PORTNAME=	cosmic-panel
DISTVERSIONPREFIX=	epoch-
DISTVERSION=	1.0.0-beta.7
CATEGORIES=	x11-wm wayland

MAINTAINER=	christian@FreeBSD.org
COMMENT=	Panel for the COSMIC desktop environment (Rust)
WWW=		https://github.com/pop-os/cosmic-panel

LICENSE=	GPLv3+
LICENSE_FILE=	${WRKSRC}/LICENSE

# Dependencies
LIB_DEPENDS=	libudev.so:devel/libudev-devd \
		libxkbcommon.so:x11/libxkbcommon

#local distfiles
DISTDIR=	${.CURDIR}/../distfiles

BUILD_DEPENDS=	rust>=1.90.0:lang/rust

USES=		cargo
USE_GITHUB=	yes

GH_ACCOUNT=	pop-os
GH_PROJECT=	cosmic-panel
GH_TAGNAME=	epoch-1.0.0-beta.7

GH_TUPLE=	pop-os:iced:c9cd78e030d5b228f190af28d908e1fbcf8737ce:iced \
		pop-os:cosmic-protocols:d0e95be25e423cfe523b11111a3666ed7aaf0dc4:cosmic_protocols

PLIST_FILES=	bin/${PORTNAME}

.if ${MACHINE_ARCH} == i386 || ${MACHINE_ARCH:Marmv?}
LTO_UNSAFE=	yes
CARGO_ENV+=	CARGO_PROFILE_RELEASE_LTO=false
.endif

# Definições dos diretórios de trabalho
ICED_WRKDIR=		${WRKDIR}/iced
COSMIC_PROTOCOLS_WRKDIR=	${WRKDIR}/cosmic_protocols

post-extract:
	@${ECHO_MSG} "=> Setting up dependencies"
	# Verifique a estrutura do cosmic-protocols
	@if [ -d "${COSMIC_PROTOCOLS_WRKDIR}/client-toolkit" ]; then \
		${ECHO_MSG} "client-toolkit found in cosmic-protocols"; \
	else \
		${ECHO_MSG} "ERROR: client-toolkit not found!"; \
		ls -la ${COSMIC_PROTOCOLS_WRKDIR}/; \
	fi

pre-configure:
	# Configure Cargo para usar as dependências locais
	@${ECHO_MSG} "=> Configuring Cargo to use local dependencies"
	${MKDIR} "${WRKSRC}/.cargo"

	# Crie um config.toml que substitui as dependências git por paths locais
	${ECHO_CMD} "[source]" > "${WRKSRC}/.cargo/config.toml"
	${ECHO_CMD} "[source.\"git+https://github.com/pop-os/iced\"]" >> "${WRKSRC}/.cargo/config.toml"
	${ECHO_CMD} "replace-with = \"local-iced\"" >> "${WRKSRC}/.cargo/config.toml"
	${ECHO_CMD} "[source.\"git+https://github.com/pop-os/cosmic-protocols\"]" >> "${WRKSRC}/.cargo/config.toml"
	${ECHO_CMD} "replace-with = \"local-cosmic-protocols\"" >> "${WRKSRC}/.cargo/config.toml"
	${ECHO_CMD} "" >> "${WRKSRC}/.cargo/config.toml"
	${ECHO_CMD} "[source.local-iced]" >> "${WRKSRC}/.cargo/config.toml"
	${ECHO_CMD} "directory = \"${ICED_WRKDIR}\"" >> "${WRKSRC}/.cargo/config.toml"
	${ECHO_CMD} "[source.local-cosmic-protocols]" >> "${WRKSRC}/.cargo/config.toml"
	${ECHO_CMD} "directory = \"${COSMIC_PROTOCOLS_WRKDIR}\"" >> "${WRKSRC}/.cargo/config.toml"

	# Também tente modificar o Cargo.toml do cosmic-protocols se necessário
	@if [ -f "${COSMIC_PROTOCOLS_WRKDIR}/Cargo.toml" ]; then \
		${ECHO_MSG} "Checking cosmic-protocols Cargo.toml structure"; \
		${CAT} "${COSMIC_PROTOCOLS_WRKDIR}/Cargo.toml" | ${HEAD} -20; \
	fi

do-install:
	${INSTALL_PROGRAM} ${WRKDIR}/target/release/${PORTNAME} \
		${STAGEDIR}${PREFIX}/bin/

.include <bsd.port.mk>

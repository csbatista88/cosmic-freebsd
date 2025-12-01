# /Home/christian/ports/cosmic-comp/Makefile
PORTNAME=	cosmic-comp
DISTVERSIONPREFIX=	epoch-
DISTVERSION=	1.0.0-beta.7
CATEGORIES=	x11-wm wayland

MAINTAINER=	christian@FreeBSD.org
COMMENT=	Compositor for the COSMIC desktop environment (Rust)
WWW=		https://github.com/pop-os/cosmic-comp

#local distfiles
DISTDIR=	${.CURDIR}/../distfiles

LICENSE=	GPLv3+
LICENSE_FILE=	${WRKSRC}/LICENSE

# Depences
LIB_DEPENDS=	libudev.so:devel/libudev-devd \
		libdisplay-info.so:sysutils/libdisplay-info \
		libseat.so:sysutils/seatd \
		libinput.so:x11/libinput \
		libxkbcommon.so:x11/libxkbcommon
RUN_DEPENDS=	mesa-dri>0:graphics/mesa-dri \
		vulkan-loader>0:graphics/vulkan-loader \
		Xwayland:x11-servers/xwayland

BUILD_DEPENDS=	rust>=1.90.0:lang/rust

USES=		cargo gl xorg
USE_GITHUB=	yes
USE_GL=		gbm
USE_XORG=	pixman

GH_ACCOUNT=	pop-os
GH_PROJECT=	cosmic-comp
GH_TAGNAME=	epoch-1.0.0-beta.7


GH_TUPLE=	pop-os:dbus-settings-bindings:b2337437d70b3db7a56211a43aa1632306711b2d:dbus_bindings \
		pop-os:libcosmic:f44d82a7e83af15270a9ca3beb832f4799699337:libcosmic \
		pop-os:iced:8cbf2b70ad229a4bc5b7055e3d0a9eef265bd10d:iced

# udev nativo
CARGO_FEATURES+=--no-default-features


PLIST_FILES=	bin/${PORTNAME} \
		share/cosmic/com.system76.CosmicSettings.Shortcuts/v1/defaults/keybindings.ron \
		share/cosmic/com.system76.CosmicSettings.WindowRules/v1/tiling_exception_defaults/tiling-exceptions.ron \
		share/wayland-sessions/${PORTNAME:S/-comp//}.desktop \

.if ${MACHINE_ARCH} == i386 || ${MACHINE_ARCH:Marmv?}
LTO_UNSAFE=	yes
CARGO_ENV+=	CARGO_PROFILE_RELEASE_LTO=false
.endif


DBUS_WRKDIR=	${WRKDIR}/dbus-settings-bindings-b2337437d70b3db7a56211a43aa1632306711b2d
LIBCOSMIC_WRKDIR=	${WRKDIR}/libcosmic-f44d82a7e83af15270a9ca3beb832f4799699337
ICED_WRKDIR=		${WRKDIR}/iced-8cbf2b70ad229a4bc5b7055e3d0a9eef265bd10d

#CARGO_VENDOR_DIR=	${WRKSRC}/cargo-crates

# =========================================================================
# tasks
# =========================================================================


post-extract:
	@${ECHO_MSG} "=> Renaming LICENSE.md to LICENSE for dbus-settings-bindings"
	@if [ -f ${DBUS_WRKDIR}/LICENSE.md ]; then \
		${MV} ${DBUS_WRKDIR}/LICENSE.md ${DBUS_WRKDIR}/LICENSE; \
	fi

	@${ECHO_MSG} "=> Copying missing 'iced' submodule into libcosmic work directory"
	@if [ -d ${ICED_WRKDIR} ]; then \
		${CP} -R ${ICED_WRKDIR}/. ${LIBCOSMIC_WRKDIR}/iced/; \
	else \
		${ECHO_MSG} "--- WARNING: ICED submodule source directory not found. ---"; \
	fi


do-install:
	${INSTALL_PROGRAM} ${WRKDIR}/target/release/${PORTNAME} \
		${STAGEDIR}${PREFIX}/bin/

post-install:
# Install extras similar to ${WRKSRC}/Makefile
.for f in ${PLIST_FILES:Nbin/*:T:S,^,data/,}
	${MKDIR} ${STAGEDIR}${PREFIX}/${PLIST_FILES:M*${f:T}:H}
	${INSTALL_DATA} ${WRKSRC}/${f} \
		${STAGEDIR}${PREFIX}/${PLIST_FILES:M*${f:T}}
.endfor


.include <bsd.port.mk>

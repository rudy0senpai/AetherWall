# CMake integration placeholder for the Qt 6 / QML Studio.

# NOTE: This file is a scaffold for Phase 1.
# Phase 0 contains only documentation, design, and prototypes.
# The Qt/QML build integration is introduced with the Studio implementation.

cmake_minimum_required(VERSION 3.20)

# Qt 6 required
find_package(Qt6 REQUIRED COMPONENTS Qml)

# Project name — Studio application (UI layer only; playback state lives in the daemon)
project(AetherWallStudio

        VERSION 0.1.0
        DESCRIPTION "AetherWall Studio — Qt 6 / QML UI"
        LANGUAGES CXX)

# Add the Studio QML module when Phase 1 begins here.
# target_add_qml_module(AetherWallStudio
#     URI AetherWall
#     QML_FILES
#         shell/Main.qml
#         pages/Home.qml
#         pages/Library.qml
#         ...)

# Link the core and runtime crates (Phase 1).
# target_link_libraries(AetherWallStudio
#     PRIVATE
#         AetherWallCore
#         AetherWallDaemon
#         AetherWallIpc
#         AetherWallPackage
#         AetherWallProviderApi)

# Install rules for the .desktop entry, icon, and translations go here.

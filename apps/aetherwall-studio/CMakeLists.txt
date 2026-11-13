# AetherWall Studio — Qt 6 / QML build
cmake_minimum_required(VERSION 3.20)
project(AetherWallStudio
    VERSION 0.1.0
    DESCRIPTION "AetherWall Studio — Qt 6 / QML UI"
    LANGUAGES CXX)

find_package(Qt6 REQUIRED COMPONENTS Core Widgets Multimedia QML Network)

set(APP_NAME aetherwall-studio)
set(APP_VERSION 0.1.0)

qt_add_executable(${APP_NAME}
    URL aetherwall-studio
    QML
        shell/Main.qml
        pages/Home.qml
        pages/Library.qml
        pages/Workshop.qml
        pages/Favorites.qml
        pages/Downloads.qml
        pages/Displays.qml
        pages/Settings.qml
        pages/Diagnostics.qml
        components/WallpaperCard.qml
        components/CategoryCard.qml
        components/Button.qml
        components/Dialog.qml
        components/SearchBar.qml
        components/LoadingIndicator.qml
        components/EmptyState.qml
        components/ErrorState.qml
        components/Preview.qml
)

target_link_libraries(${APP_NAME}
    PRIVATE Qt6::Core
            Qt6::Widgets
            Qt6::Multimedia
            Qt6::QML
            Qt6::Network
)

install(TARGETS ${APP_NAME} DESTINATION bin)
install(FILES aetherwall-studio.desktop DESTINATION share/applications)

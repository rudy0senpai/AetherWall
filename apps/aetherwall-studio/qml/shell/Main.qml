// AetherWall Studio — main application shell
// Qt 6 / QML
// Design system: Phase 0 (dark graphite/black, purple/magenta accents)

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import org.kde.PlasmaCore 1.0 as Plasma

Rectangle {
    id: root
    width: 1440
    height: 900
    color: "#0B0B10"

    // ── Desktop preview (top section) ──
    // Actual wallpaper preview — not a system-monitoring overlay
    WallpaperPreview {
        id: preview
        anchors {
            top: parent.top
            left: parent.left
            right: parent.right
            height: 240
        }
        display: DisplaysModel.displayList[DisplaysModel.selectedDisplayIndex]
        fitMode: SettingsStore.defaultFitMode
    }

    // ── Studio area (lower section) ──
    Column {
        id: studio
        anchors {
            left: parent.left
            right: parent.right
            bottom: parent.bottom
        }

        // ── Navigation ──
        NavigationView {
            id: nav
            width: 154
            height: studio.height
            activeView: NavigationModel.activeView

            NavigationItem { id: nHome; title: "Home"; icon: "house"; target: homeView }
            NavigationItem { id: nLibrary; title: "Library"; icon: "grid"; target: libraryView }
            NavigationItem { id: nWorkshop; title: "Workshop"; icon: "tool"; target: workshopView }
            NavigationItem { id: nFavorites; title: "Favorites"; icon: "star"; target: favoritesView }
            NavigationItem { id: nDownloads; title: "Downloads"; icon: "download"; target: downloadsView }
            NavigationItem { id: nDisplays; title: "Displays"; icon: "monitors"; target: displaysView }
            NavigationItem { id: nSettings; title: "Settings"; icon: "settings"; target: settingsView }
            NavigationItem { id: nDiagnostics; title: "Diagnostics"; icon: "bug"; target: diagnosticsView }
        }

        // ── Category chips ──
        CategoryChips {
            id: categoryChips
            anchors {
                top: nav.bottom
                left: nav.left
                right: nav.right
                bottom: contentArea.top
            }
            categories: [
                { name: "Pictures", count: 24 },
                { name: "Slideshow", count: 3 },
                { name: "Videos", count: 12 },
                { name: "Web Wallpapers", count: 8 },
                { name: "Anime Pictures", count: 56 },
                { name: "Anime Videos", count: 21 },
                { name: "3D Wallpapers", count: 14 },
                { name: "Wallpaper Games", count: 9 }
            ]
        }

        // ── Main content area ──
        Component {
            id: contentArea
            StackView {
                id: stack
                anchors {
                    left: parent.left
                    right: parent.right
                    bottom: parent.bottom
                }
                focus: true
            }
        }
    }
}

// Library view — Studio page
import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

Item {
    id: root
    width: parent.width
    height: parent.height

    Column {
        id: column
        anchors.fill: parent
        anchors.margins: 16

        // Filter bar with search
        Row {
            spacing: 8
            Column {
                width: 200
                ToolBar {
                    Label { text: "Library"; font.pixelSize: 16; color: "#F5F3FA"; }
                }
            }
            SearchBar {
                placeholderText: "Search library…"
                onActivated: {
                    // Filter grid by query
                    filterText = text
                }
            }
            Button {
                text: "Import folder"
                background: "#14131B"
                border.color: "#343044"
            }
            Button {
                text: "Reset filters"
                background: "#14131B"
                border.color: "#343044"
            }
        }

        // Category filter chips
        Row {
            id: categoryRow
            spacing: 8
            Repeater {
                model: ["Pictures", "Slideshow", "Videos", "Web Wallpapers", "Anime Pictures", "Anime Videos", "3D Wallpapers", "Wallpaper Games"]
                Button {
                    text: modelData
                    background: "#14131B"
                    border.color: "#343044"
                    PropertyChanges { when: selectedCategory === modelData; border.color: "#8B3DBB"; backgroundColor: "#1D1A27" }
                }
            }
        }

        // Media grid
        GridView {
            id: mediaGrid
            anchors.fill: parent
            anchors.marginsTop: 8
            cellWidth: 180
            cellHeight: 120
            delegate: WallpaperCard {
                wallpaper: modelData
                onClicked: {
                    // Navigate to wallpaper details
                   StackView.restore(items.indexOf(controller.detailView), parent.width, parent.height)
                }
            }
        }
    }
}

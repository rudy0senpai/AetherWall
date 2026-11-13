// WallpaperCard — reusable wallpaper thumbnail card
import QtQuick
import QtQuick.Controls

Item {
    id: root
    property alias wallpaper: card
    property var wallpaperData: null
    signal clicked()

    width: 180
    height: 140

    Rectangle {
        id: card
        anchors.fill: parent
        radius: 8
        color: "#14131B"
        border.color: "#343044"
        border.width: 1

        Text {
            id: thumbnail
            text: "📷"
            font.pixelSize: 40
            anchors.left: parent.left
            anchors.top: parent.top
            anchors.margins: 8
        }

        Column {
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.bottom: parent.bottom
            anchors.margins: 8
            spacing: 4

            Text {
                text: wallpaperData?.title ?? "Wallpaper"
                font.pixelSize: 13
                font.weight: Font.Medium
                color: "#F5F3FA"
                elide: Text.ElideRight
            }
            Text {
                text: (wallpaperData?.width ?? 0) > 0 ? (wallpaperData.width + "×" + wallpaperData.height) : ""
                font.pixelSize: 11
                color: "#AAA5B8"
            }
            Row {
                spacing: 8
                Label {
                    text: "♥"
                    font.pixelSize: 14
                    color: wallpaperData?.is_favorite ?? false ? "#D946EF" : "#7A728A"
                    anchors.right: parent.right
                }
                Button {
                    text: "Apply"
                    background: "#14131B"
                    border.color: "#343044"
                    anchors.right: parent.right
                }
            }
        }
    }

    TapHandler {
        onClicked: clicked()
    }
}

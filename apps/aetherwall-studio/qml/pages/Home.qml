// Home view — Studio page
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

        // Header
        Row {
            id: header
            anchors.horizontalFill: true
            spacing: 8
            Label {
                text: "Home"
                font.pixelSize: 20
                font.weight: Font.Medium
                color: "#F5F3FA"
            }
            Button {
                id: searchButton
                text: "Search wallpapers…"
                background: "#14131B"
                border.color: "#343044"
                highlighted: true
            }
        }

        // Featured wallpaper
        Item {
            id: featured
            height: 220
            background: Rectangle {
                anchors.fill: parent
                color: "#1A1230"
                border.color: "#343044"
                radius: 8
            }
            Label {
                anchors.centerIn: parent
                text: "Neon Nebula Drift"
                font.pixelSize: 24
                font.weight: Font.SemiBold
                color: "#F5F3FA"
            }
            Label {
                anchors.topMargin: 8
                anchors.horizontalCenter: parent.horizontalCenter
                text: "A slowly animated nebula scene."
                font.pixelSize: 14
                color: "#AAA5B8"
            }
            Button {
                anchors {
                    right: parent.right
                    bottom: parent.bottom
                    bottomMargin: 8
                    left: parent.left
                    horizontalCenter: parent.horizontalCenter
                    horizontalCenterOffset: -8
                }
                text: "View details →"
                background: "#8B3DBB"
                color: "#F5F3FA"
            }
        }

        // Quick-start category cards
        GridView {
            id: quickStartGrid
            anchors.fill: parent
            anchors.marginsTop: 16
            cellsPerRow: 4
            flow: GridView.LeftToRight
            delegate: Column {
                spacing: 8
                Rectangle {
                    width: parent.width
                    height: 90
                    radius: 8
                    color: "#14131B"
                    border.color: "#343044"
                    Text {
                        anchors.centerIn: parent
                        text: "📷 Pictures"
                        font.pixelSize: 13
                        color: "#F5F3FA"
                    }
                    Text {
                        anchors.topMargin: 4
                        anchors.left: parent.left
                        anchors.horizontalCenter: parent.horizontalCenter
                        text: "24 items"
                        font.pixelSize: 11
                        color: "#AAA5B8"
                    }
                }
            }
        }
    }
}

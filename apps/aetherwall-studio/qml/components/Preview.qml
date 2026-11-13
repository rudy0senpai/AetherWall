// Preview — image/video/web preview component
import QtQuick
import QtQuick.Controls

Item {
    id: root
    property alias source: preview.source
    property bool showControls: true
    width: 300
    height: 180

    Rectangle {
        id: container
        anchors.fill: parent
        radius: 8
        color: "#14131B"
        border.color: "#343044"
        border.width: 1

        VideoOutput {
            id: preview
            anchors.fill: parent
            opacity: showControls ? 0.9 : 1
        }

        Rectangle {
            id: placeholder
            anchors.fill: parent
            radius: 8
            color: "#1A1230"
            Label {
                text: "Preview"
                font.pixelSize: 14
                color: "#AAA5B8"
                anchors.centerIn: parent
            }
        }

        if (showControls) {
            Rectangle {
                id: controls
                anchors.bottom: parent.bottom
                anchors.horizontalCenter: parent.horizontalCenter
                width: parent.width
                height: 40
                color: "#0F0E16"
                opacity: 0.8
                Label {
                    text: "⏸"
                    font.pixelSize: 16
                    color: "#F5F3FA"
                    anchors.centerIn: parent
                }
            }
        }
    }
}

// Favorites view — Studio page
import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

Item {
    id: root
    width: parent.width
    height: parent.height
    Column {
        anchors.fill: parent
        anchors.margins: 16
        ToolBar {
            Label { text: "Favorites"; font.pixelSize: 16; color: "#F5F3FA"; }
            Spacer {}
            Button { text: "Clear all"; background: "#14131B"; border.color: "#343044"; }
        }
        Item {
            height: 200
            Label { text: "★ No favorites yet"; font.pixelSize: 18; color: "#AAA5B8"; anchors.centerIn: parent; }
            Label { text: "Tap the heart to save a wallpaper here."; font.pixelSize: 14; color: "#7A728A"; anchors.topMargin: 8; anchors.horizontalCenter: parent.horizontalCenter; }
            Button { text: "Discover wallpapers"; background: "#8B3DBB"; color: "#F5F3FA"; anchors.topMargin: 16; anchors.horizontalCenter: parent.horizontalCenter; }
        }
    }
}

// CategoryCard — category filter/discovery card
import QtQuick
import QtQuick.Controls

Item {
    id: root
    property string title: "Category"
    property int count: 0
    signal clicked()

    width: 180
    height: 90

    Rectangle {
        anchors.fill: parent
        radius: 8
        color: "#14131B"
        border.color: "#343044"
        border.width: 1

        Text {
            text: title
            font.pixelSize: 14
            font.weight: Font.Medium
            color: "#F5F3FA"
            anchors.left: parent.left
            anchors.top: parent.top
            anchors.margins: 8
        }
        Text {
            text: count + " items"
            font.pixelSize: 11
            color: "#AAA5B8"
            anchors.topMargin: 4
            anchors.right: parent.right
            anchors.verticalCenter: parent.verticalCenter
        }
    }

    TapHandler {
        onClicked: clicked()
    }
}

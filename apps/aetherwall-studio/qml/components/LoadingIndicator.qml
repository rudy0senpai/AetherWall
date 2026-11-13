// LoadingIndicator — spinner with label
import QtQuick
import QtQuick.Controls

Item {
    id: root
    property alias text: label.text
    width: 120
    height: 40

    Rectangle {
        anchors.fill: parent
        color: "#14131B"
        radius: 8
    }

    RotatingSquare {
        id: spinner
        width: 20
        height: 20
        color: "#8B3DBB"
        anchors.centerIn: parent
    }

    Label {
        id: label
        text: "Loading…"
        font.pixelSize: 12
        color: "#AAA5B8"
        anchors.centerIn: parent
    }
}

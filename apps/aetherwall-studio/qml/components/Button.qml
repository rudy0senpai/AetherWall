// Button — primary UI button with states
import QtQuick
import QtQuick.Controls

Button {
    id: button
    text: "Button"
    background: "#14131B"
    border.color: "#343044"
    border.width: 1
    radius: 8
    padding: 8
    font.pixelSize: 13
    font.weight: Font.Medium

    property string variant: "default"
    property bool enabled: true
    property alias textColor: label.color

    Label {
        id: label
        anchorCenterIn: parent
        color: "#F5F3FA"
    }

    states: [
        State {
            name: "hovered"
            PropertyChanges { background: "#1D1A27"; border.color: "#4A435C" }
        },
        State {
            name: "down"
            PropertyChanges { background: "#0F0E16"; border.color: "#2A2538" }
        },
        State {
            name: "disabled"
            Opacity: 0.5
            enabled: false
        }
    ]

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        onPressed: button.down = true
        onReleased: button.down = false

        onWheel: { /* scroll handled by parent */ }
    }
}

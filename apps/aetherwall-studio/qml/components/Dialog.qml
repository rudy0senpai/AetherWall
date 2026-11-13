// Dialog — modal dialog
import QtQuick
import QtQuick.Controls

Dialog {
    id: dialog
    title: "Confirm"
    text: "Are you sure you want to remove this wallpaper?"
    standardButtons: Dialog.Close | Dialog.Apply
    Modal: true
}

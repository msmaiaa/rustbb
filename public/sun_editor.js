var editors = {};

export function create_editor(textarea_id) {
	let fullId = `suneditor_${textarea_id}`;
	while (!document.getElementById(textarea_id) && !SUNEDITOR) { }
	if (document.getElementById(fullId) || editors[textarea_id]) {
		return false;
	}
	setTimeout(() => {
		editors[textarea_id] = SUNEDITOR.create((document.getElementById(textarea_id)), {
			// All of the plugins are loaded in the "window.SUNEDITOR" object in dist/suneditor.min.js file
			// Insert options
			// Language global object (default: en)
			lang: SUNEDITOR_LANG['en']
		});
		return true;
	}, 50);
}

export function get_editor_text(textarea_id) {
	//	TODO: try catch
	let editor = editors[textarea_id];
	if (!editor) {
		return "";
	}
	return editor.getText();
}
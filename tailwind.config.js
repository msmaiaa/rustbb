/** @type {import('tailwindcss').Config} */
module.exports = {
	content: {
		files: ["*.html", "./src/**/*.rs"],
	},
	theme: {
		extend: {
			colors: {
				bg_primary: "#2D3238",
				bg_darker: "#15171a",
				text_primary: "#fff",
				header: "#081C15"
			},
		},
	},
	plugins: [],
}
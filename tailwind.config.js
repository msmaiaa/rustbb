/** @type {import('tailwindcss').Config} */
module.exports = {
	content: {
		files: ["*.html", "./src/**/*.rs"],
	},
	theme: {
		extend: {
			colors: {
				bg_primary: "#0a0908",
				text_primary: "#eae0d5",
				header: "#081C15"
			},
		},
	},
	plugins: [],
}
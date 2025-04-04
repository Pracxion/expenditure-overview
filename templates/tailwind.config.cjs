const { fontFamily } = require("tailwindcss/defaultTheme");

/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ["./src/**/*.html"],
	theme: {
		extend: {
			fontFamily: {
				sans: ["Inter var", ...fontFamily.sans],
			},
		},
	},
};

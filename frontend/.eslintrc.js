module.exports = {
    "env": {
        "browser": true,
        "es6": true
    },
    "extends": "airbnb",
    "parserOptions": {
        "ecmaFeatures": {
            "experimentalObjectRestSpread": true,
            "jsx": true
        },
        "sourceType": "module"
    },
    "plugins": [
        "react"
    ],
    "rules": {
        "react/jsx-filename-extension": "off",
        "func-names": "off",
        "react/forbid-prop-types": "off",
        "jsx-a11y/anchor-is-valid": "off", // gets mad at '<Link to=' even though that's fine
    },
};

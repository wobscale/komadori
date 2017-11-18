import React, { Component } from 'react';

class GithubLogin extends Component {
    constructor(props) {
        super(props);
        this.state = {
            loggingIn: false,
        };
        this.loginClick = this.loginClick.bind(this);
    }

    loginClick() {
        this.setState(prev => ({
            loggingIn: true,
        }));
        // Stolen partially from 
        // https://github.com/rust-lang/crates.io/blob/e8cae0e872be4edf02f0876db4e85c082e70ecc9/app/routes/login.js#L14-L40
        // Used and modified under the terms of the MIT license.

        // TODO:
        // 1. Open window at a new route for github login flow
        // 2. retrieve oauth response from the window
        // 3. Get the user
        // 4. Route to dashboard or create user depending on 3.'s result
    }

    render() {
        if(this.state.loggingIn) {
            return (
                <div class="logging-in">
                    <p>Please complete the login in the new window...</p>
                </div>
            );
        } else {
            return (
                <button onClick={this.loginClick}>
                    Login with Github
                </button>
            );
        }
    }
};

export default GithubLogin;

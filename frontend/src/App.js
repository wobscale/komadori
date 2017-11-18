import React, { Component } from 'react';

import GithubLogin from './github-login-component';

import logo from './logo.svg';
import './App.css';

class App extends Component {
  render() {
    return (
      <div className="Accounts">
        <header className="Wobscale-Login">
          <img src={logo} className="Wobscale-logo" alt="logo" />
          <h1 className="App-title">Login</h1>
        </header>
        <p className="App-intro">
          Login to your Wobscale Account
        </p>
        <GithubLogin />

        <h2>Create an account</h2>
        <p> If you don't have an account yet, login to create one </p>
      </div>
    );
  }
}

export default App;

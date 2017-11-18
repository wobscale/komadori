import React, { Component } from 'react';
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

        <h2>Create an account</h2>
        <p> If you don't have an account yet, you can create one</p>
      </div>
    );
  }
}

export default App;

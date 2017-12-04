import { withRouter } from 'react-router';
import React, { Component } from 'react';
import {
  BrowserRouter as Router,
  Route,
  Link
} from 'react-router-dom'
import config from './config';
import qs from 'query-string';

export class CreateAccount extends Component {
    constructor(props) {
        super(props);
        this.state = {
            step: 0,
        };
        this.createClick = this.createClick.bind(this);
    }

    createClick() {
        this.setState(prev => ({
            step: 1,
        }));
    }

    render() {
        if(this.state.step === 0 && this.props.location.state && this.props.location.state.partialUser) {
            let pu = this.props.location.state.partialUser;
            return (
                <div className="create-account">
                  <p> Complete this form to create a Wobscale Account associated with your {pu.provider} account '{pu.login}'.</p>
                  <button onClick={this.createClick}>
                    Create
                  </button>
                </div>
            );
        } else if (this.state.step === 1) {
            return (
                <div className="logging-in">
                    <p>Creating account...</p>
                </div>
            );
        } else {
            return (
                <div>
                  <h1>This shouldn't happen!</h1>
                </div>
            );
        }
    }
};

export default withRouter(CreateAccount);

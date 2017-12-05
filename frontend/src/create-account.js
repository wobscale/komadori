import { withRouter } from 'react-router';
import { Redirect } from 'react-router'
import React, { Component } from 'react';
import {
  BrowserRouter as Router,
  Route,
  Link
} from 'react-router-dom'
import config from './config';
import qs from 'query-string';

const steps = {
  error: 'error',
  default: 'default',
  creating: 'creating',
  created: 'created',
};
  


export class CreateAccount extends Component {
    constructor(props) {
        super(props);
        let pu = this.props.location.state && this.props.location.state.partialUser;
        this.state = {
            step: steps.default,
            email: '',
            username: '',
            partialUser: pu,
        };
        this.handleCreate = this.handleCreate.bind(this);
        this.handleInputChange = this.handleInputChange.bind(this);
    }

    handleInputChange(event) {
      this.setState({
        [event.target.name]: event.target.value
      });
    }

    handleCreate() {
        this.setState(prev => ({
            step: steps.creating,
        }));
        // validate
        if (this.state.email === '') {
          this.setState(prev => ({
              step: steps.error,
              err: "email is required",
          }));
          return;
        }
        if (this.state.username === '') {
          this.setState(prev => ({
              step: steps.error,
              err: "username is required",
          }));
          return;
        }

        fetch(config.api + "/user/create", {
          mode: 'cors',
          headers: {"content-type": "application/json"},
          credentials: 'include',
          method: "POST",
          body: JSON.stringify({
            username: this.state.username,
            email: this.state.email,
            partial_user: this.state.partialUser,
          }),
        }).then((resp) => resp.json()).then((resp) => {
          if(resp.Ok && resp.Ok.uuid) {
            this.setState({
              step: steps.created,
              uuid: resp.Ok.uuid,
              username: resp.Ok.username,
            });
          } else {
            this.setState({
              step: steps.error,
              err: resp.Err && resp.Err.message || "unknown error",
            });
          }
        });
    }

    render() {
      switch(this.state.step) {
        case steps.error:
          return (
            <div className="error">
              <h2> Error </h2>
              <p> {this.state.err} </p>
              <button onClick={() => { this.setState({step: steps.default}) }}>
                Retry
              </button>
            </div>
          );
          break;
        case steps.default:
          if (this.state.partialUser) {
              let pu = this.state.partialUser;
              return (
                  <div className="create-account">
                    <p> Complete this form to create a Wobscale Account associated with your {pu.provider} account '{pu.provider_name}'.</p>
                    <form onSubmit={this.handleCreate}>
                      <label>
                      Username:
                      <input type="text"
                        onChange={this.handleInputChange}
                        name="username" />
                      </label>
                      <br />

                      <label>
                      Email:
                      <input type="text"
                        onChange={this.handleInputChange}
                        name="email" />
                      </label>
                      <br />

                      <input type="submit" value="Create" />
                    </form>
                  </div>
              );
            } else {
              // user got here without having a partial user, presumably this
              // is a browser history or f5 or such. Force em through the flow again.
              return <Redirect to="/"/>
            }
            break;
          case steps.creating:
            console.log("create");
            return (
                <div className="logging-in">
                    <p>Creating account...</p>
                </div>
            );
            break;
          case steps.created:
            console.log("created");
            return (
                <div className="created-in">
                    <p>Created account {this.state.username}.</p>
                    <b><p>Congratulations, you're the first user to get {this.state.uuid} as your UUID!</p></b>
                </div>
            );
            break;
      }
      // This shouldn't happen :)
      alert(`Step was: ${this.state.step}`);
      return <Redirect to="/"/>
    }
};

export default withRouter(CreateAccount);

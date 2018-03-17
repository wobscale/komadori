import React from 'react';
import { withRouter, Redirect } from 'react-router';

const steps = {
  error: 'error',
  default: 'default',
  creating: 'creating',
  created: 'created',
};

const CreateAccount = (createStep) => {
  switch (createStep) {
    case steps.default:
      if (this.state.partialUser) {
        const pu = this.state.partialUser;
        return (
          <div className="create-account">
            <p>
              Complete this form to create a Wobscale Account associated
              with your {pu.provider} account {`'${pu.provider_name}'`}.
            </p>
            <form onSubmit={this.handleCreate}>
              <label htmlFor="username">
                  Username:
                  <input
                    type="text"
                    onChange={this.handleInputChange}
                    name="username"
                  />
              </label>
              <br />

              <label htmlFor="email">
                  Email:
                  <input
                    type="text"
                    onChange={this.handleInputChange}
                    name="email"
                  />
              </label>
              <br />

              <input type="submit" value="Create" />
            </form>
          </div>
        );
      }
      // user got here without having a partial user, presumably this
      // is a browser history or f5 or such. Force em through the flow again.
      return <Redirect to="/" />;
    case steps.creating:
      return (
        <div className="logging-in">
          <p>Creating account...</p>
        </div>
      );
    case steps.created:
      return (
        <div className="created-in">
          <p>Created account {this.state.username}.</p>
        </div>
      );
    default:
      return <Redirect to="/" />;
  }
};

export default withRouter(CreateAccount);

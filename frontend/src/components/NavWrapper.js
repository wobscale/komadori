import React from 'react';
import PropTypes from 'prop-types';
import { Route, Link, Switch } from 'react-router-dom';
import Topbar from './Topbar';
import Security from '../containers/Security';
import userUtil from '../util/user';
import UserDashboard from '../containers/UserDashboardContainer';
import AdminManageUsers from '../containers/AdminManageUsers';

const NavWrapper = ({ user }) => (
  <div className="wrapper">
    <Topbar user={user} />
    <div className="main-content">
      <div className="leftbar">
        <nav className="menu">
          <span>Sections</span> <br /> <br />
          <Link to="/user/dashboard">Dashboard</Link> <br />
          <Link to="/user/security">Security</Link> <br />
          {
            userUtil.userIsAdmin(user) ?
              <div className="admin-nav-section">
                <br />
                <span>Admin Sections</span> <br />
                <Link to="/admin/users">Manage Users</Link>
              </div>
            : null
          }
        </nav>
      </div>
      <div className="main">
        <Switch>
          <Route path="/admin/users" component={AdminManageUsers} />
          <Route path="/user/security" component={Security} />
          <Route path="/" component={UserDashboard} />
        </Switch>
      </div>
    </div>
  </div>
);
NavWrapper.propTypes = {
  user: PropTypes.object.isRequired,
};

export default NavWrapper;

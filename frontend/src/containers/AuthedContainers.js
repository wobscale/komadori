import { connectedRouterRedirect } from 'redux-auth-wrapper/history4/redirect';
import locationHelperBuilder from 'redux-auth-wrapper/history4/locationHelper';
import OldLoginPage from '../components/LoginPage';
import OldGithubLogin from './LoginWithGithubContainer';
import OldUserConsent from '../user-consent';
import OldCreateAccount from './CreateAccount';
import OldUserDashboard from './UserDashboardContainer';

const userIsAuthenticated = connectedRouterRedirect({
  redirectPath: '/',
  authenticatedSelector: state => state.user && state.user.loggedIn,
  wrapperDisplayName: 'UserIsAuthenticated',
});

const locationHelper = locationHelperBuilder();

const userIsNotAuthenticated = connectedRouterRedirect({
  redirectPath: (state, ownProps) => {
    if (state.user && state.user.loggedIn) {
      return locationHelper.getRedirectQueryParam(ownProps) || '/user/dashboard';
    }
    if (state.partialUser.partialUser) {
      return '/account/create';
    }
    return '/user/dashboard';
  },
  allowRedirectBack: false,
  authenticatedSelector: (state) => {
    if (state.user && state.user.loggedIn) {
      return false;
    }
    return true;
  },
  wrapperDisplayName: 'UserIsNotAuthenticated',
});

const userIsNotPartial = connectedRouterRedirect({
  redirectPath: () => '/account/create',
  allowRedirectBack: false,
  authenticatedSelector: (state) => {
    if (state.partialUser && state.partialUser.partialUser) {
      return false;
    }
    return true;
  },
  wrapperDisplayName: 'UserIsPartial',
});

export const Login = userIsNotAuthenticated(OldLoginPage);
export const CreateAccount = userIsNotAuthenticated(OldCreateAccount);

export const GithubLogin = userIsNotAuthenticated(userIsNotPartial(OldGithubLogin));

export const UserDashboard = userIsAuthenticated(OldUserDashboard);
export const UserConsent = userIsAuthenticated(OldUserConsent);

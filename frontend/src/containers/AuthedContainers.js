import { connectedRouterRedirect } from 'redux-auth-wrapper/history4/redirect';
import locationHelperBuilder from 'redux-auth-wrapper/history4/locationHelper';
import OldLoginPage from '../components/LoginPage';
import OldGithubLogin from './LoginWithGithubContainer';
import OldUserConsent from './ConsentContainer';
import OldCreateAccount from './CreateAccount';
import OldUserDashboard from './UserDashboardContainer';
import OldNavWrapper from '../components/NavWrapper';
import OldBootstrapAdmin from './AdminBootstrapContainer';

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

const userIsNotAdmin = connectedRouterRedirect({
  redirectPath: '/',
  allowRedirectBack: false,
  authenticatedSelector: state => !(state.user && state.user.loggedIn && state.user.user && state.user.user.groups.includes('admins')),
  wrapperDisplayName: 'UserIsNotAdmin',
});

export const Login = userIsNotAuthenticated(OldLoginPage);
export const CreateAccount = userIsNotAuthenticated(OldCreateAccount);

export const GithubLogin = userIsNotAuthenticated(userIsNotPartial(OldGithubLogin));

export const UserDashboard = userIsAuthenticated(OldUserDashboard);
export const NavWrapper = userIsAuthenticated(OldNavWrapper);
export const UserConsent = userIsAuthenticated(OldUserConsent);
export const BootstrapAdmin = userIsAuthenticated(userIsNotAdmin(OldBootstrapAdmin));

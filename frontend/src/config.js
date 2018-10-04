function providers() {
  const p = ['github'];
  if (process.env.NODE_ENV === 'development') {
    p.push('local');
  }
  return p;
}

function api() {
  if (process.env.NODE_ENV === 'development') {
    return 'http://localhost:8000';
  }
  // canonically the backend is hosted at 'komadori.'
  return `${window.location.protocol}//komadori.${window.location.host}`;
}

export default {
  api: api(),
  // Well known admin group uuid
  adminGroup: 'b249560fc7c2463a872a79c9841d0139',
  loginProviders: providers(),
};

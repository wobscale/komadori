function providers() {
  const p = ['github'];
  if (process.env.NODE_ENV === 'development') {
    p.push('local');
  }
  return p;
}

export default {
  api: 'http://localhost:8000',
  // Well known admin group uuid
  adminGroup: 'b249560fc7c2463a872a79c9841d0139',
  loginProviders: providers(),
};

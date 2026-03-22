import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  vus: 3,
  duration: '5m',
};

export default function () {
  const isError = Math.random() < 0.10;

  let url;
  if (isError) {
    url = 'http://localhost:8080/invalid';
  } else {
    url = 'http://localhost:8080/valid';
  }

  const res = http.get(url, { redirects: 0 });

  if (isError) {
    check(res, { 'status is 404': (r) => r.status === 404 });
  } else {
    check(res, { 'status is 307': (r) => r.status === 307 });
  }

  sleep(1);
}

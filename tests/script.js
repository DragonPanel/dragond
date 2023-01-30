import { check, group } from 'k6';
import http from 'k6/http';

const base_endpoint = 'http://localhost:4444';
const only200callback = http.expectedStatuses(300);

export default function () {
    group('systemd', systemd);
    group('journald', journald);
}

function systemd() {
    group('list-units', systemd_list_units);
    group('load-unit', systemd_load_unit);
}

function systemd_list_units() {
    let res = http.get(base_endpoint + '/systemd' + '/list-units', { responseCallback: only200callback });

    test_if_json(res);

    check(res, {
        'is journald service present': (r) => {
            const body = r.json();

            return body.some(
                (unit) => {
                    if(unit.name == 'systemd-journald.service') {
                        return true;
                    }
                }
            );
        },
    });
}

function systemd_load_unit() {
    let res = http.get(base_endpoint + '/systemd' + '/load-unit/systemd-journald.service', { responseCallback: only200callback });

    test_if_json(res);

    check(res, {
        'is journald service loaded': (r) => {
            const body = r.json();

            return body.loadState == 'loaded';
        },
    });
}

function journald() {
    group('unit-logs', journald_unit_logs);
}

function journald_unit_logs() {
}

function test_if_json(res) {
    check(res, {
        'is Content-Type set to json': (r) => {
            const contentType = r.headers['Content-Type'];
            if(contentType == 'application/json') {
                return true;
            } else {
                console.error('content-type was', contentType);
                return false;
            }
        },
        'is body a valid json': (r) => {
            const bodyJsoned = r.json();
            if(typeof bodyJsoned == 'object') {
                return true;
            } else {
                console.error('was unable to parse body as JSON');
                return false;
            }
        },
    });
}

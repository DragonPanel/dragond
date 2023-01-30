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

    let invalid_unit_res = http.get(base_endpoint + '/systemd' + '/load-unit/non-existing.service');
    check(invalid_unit_res, {
        'is return code 404 when loading non existing unit': (r) => {
            return res.status == 404;
        },
    });
}

function journald() {
    group('unit-logs', journald_unit_logs);
}

function journald_unit_logs() {
    let without_query_res = http.get(base_endpoint + '/journald' + '/unit-logs/systemd-journald.service', { responseCallback: only200callback });

    test_if_json(without_query_res);
    check(without_query_res, {
        'does request without query params return only one entry': (r) => {
            return r.length == 1;
        },
    });

    let five_latest_res = http.get(base_endpoint + '/journald' + '/unit-logs/systemd-journald.service?lines_number=5', { responseCallback: only200callback });
    test_if_json(five_latest_res);
    check(without_query_res, {
        'does request with lines_num=5 return five entries': (r) => {
            return r.length == 5;
        },
    });
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
            let body_as_json;
            try {
                body_as_json = r.json();
            } catch(e) {
                return false;
            }

            return typeof body_as_json == 'object';
        },
    });
}

/**
 * Creates a query string from query obect
 * @param {String} queryObject      Object with query keys and values
 * @returns {String}                Query String
 */
export function getQueryString(queryObject) {
    if (Object.keys(queryObject).length == 0) return ""
    let queryString = "?"
    for (const [key, value] of Object.entries(queryObject)) {
        queryString += `${key}=${value}&`
    }
    return queryString.substring(0, queryString.length - 1);
}

/**
 * Makes an authenticated request to the OpenCourts API using fetch()
 * @param {String} path             The Location of the endpoint or the Ressource which is to fetch
 * @param {String} method           The HTTP method ([GET], POST, PUT, DELETE)
 * @param {Object} headers          Additional headers to Authorization and Content-Type
 * @param {Object} body             Body data to send
 * @returns {Object}                Response
 */
export async function makeJsonRequest(path, method = "GET", body = null, headers = null) {

    const reqPath = getPath(path)
    const reqMethod = getMethod(method)
    const reqBody = getBody(body, reqMethod)
    const reqHeaders = getHeaders(headers, reqMethod)

    let response
    response = await getResponse(reqPath, reqMethod, reqHeaders, reqBody)

    try {
        return await response.json();
    }
    catch (_) {
        return true // response is ok but empty
    }
}

async function getResponse(path, method, headers, body) {
    const requestOptions = { method: method, headers: headers }
    if (body) { requestOptions['body'] = JSON.stringify(body) }

    let response = await fetch(process.env.VUE_APP_HOTELS_API_LOCATION + path, requestOptions);

    if (!response.ok) { throw new Error(response.statusText) }
    return response
}

function getPath(path) {
    let reqPath = path;
    if (reqPath[0] !== '/') {
        reqPath = '/' + reqPath;
    }

    return reqPath
}

function getMethod(method) {
    let reqMethod = method.toUpperCase();
    if (!['GET', 'POST', 'PUT', 'DELETE'].includes(reqMethod)) {
        reqMethod = 'GET'
    }

    return reqMethod
}

function getBody(body, method) {
    return (['PUT', 'POST'].includes(method)) ? body : null
}

function getHeaders(headers, method) {
    let reqHeaders = {
        Accept: 'application/json',
    };

    if (['PUT', 'POST'].includes(method)) {
        reqHeaders['Content-Type'] = 'application/json';
    }

    if (headers) { reqHeaders = { ...reqHeaders, headers } }

    return reqHeaders
}
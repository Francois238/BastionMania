export interface Jwt {
    id : string,
    mail : string,
    admin: boolean,
    otp: boolean | null,
    complete_authentication: boolean,
    iat: number,
    exp: number
}

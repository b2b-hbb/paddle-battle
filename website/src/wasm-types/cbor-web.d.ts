declare module 'cbor-web' {
    export interface DecodedValue extends Map<number, any> {
        value: any;
    }

    export function decode(data: Uint8Array): DecodedValue;
}

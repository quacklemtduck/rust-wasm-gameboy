export const encodeBase64 = (array: Uint8Array) => {
    let binaryString = '';
    array.forEach((byte) => {
        binaryString += String.fromCharCode(byte);
    });
    return window.btoa(binaryString);
}

export const decodeBase64 = (base64String: string) => {
    let binaryString = window.atob(base64String);
    const length = binaryString.length;
    const bytes = new Uint8Array(length);
    for (let i = 0; i < length; i++) {
      bytes[i] = binaryString.charCodeAt(i);
    }
    return bytes;
}
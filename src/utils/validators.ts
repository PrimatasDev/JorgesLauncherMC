const MINECRAFT_USERNAME_REGEX = /^[a-zA-Z0-9_]{3,16}$/;

export function validateUsername(username: string): boolean {
  return MINECRAFT_USERNAME_REGEX.test(username);
}

export function sanitizeUsernameInput(value: string): string {
  return value.replace(/[^a-zA-Z0-9_]/g, "");
}

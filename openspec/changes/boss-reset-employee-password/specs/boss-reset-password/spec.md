## ADDED Requirements

### Requirement: Boss can reset staff password via API
The system SHALL provide a `PUT /api/staff/{id}/password` endpoint that allows a boss to reset the password of a staff member in their workshop.

#### Scenario: Successful password reset
- **WHEN** a boss sends `PUT /api/staff/{id}/password` with `{ "newPassword": "abc123" }` for a staff in their workshop
- **THEN** the staff's password hash SHALL be updated with the Argon2 hash of the new password
- **AND** the response SHALL return `{ "code": 0 }`

#### Scenario: Staff not in boss's workshop
- **WHEN** a boss sends `PUT /api/staff/{id}/password` for a staff NOT in their workshop
- **THEN** the system SHALL return a 404 error

#### Scenario: Target user is not staff role
- **WHEN** a boss sends `PUT /api/staff/{id}/password` for a user with role `boss`
- **THEN** the system SHALL return a 403 error "只能重置员工密码"

#### Scenario: Non-boss user attempts reset
- **WHEN** a staff user sends `PUT /api/staff/{id}/password`
- **THEN** the system SHALL return a 403 error (require_boss check)

#### Scenario: Empty password
- **WHEN** a boss sends `PUT /api/staff/{id}/password` with an empty `newPassword`
- **THEN** the system SHALL return a 400 error "新密码不能为空"

### Requirement: Staff management UI shows reset password action
The staff list page (`/_auth/_boss/staff/`) SHALL include a "重置密码" action for each staff member.

#### Scenario: Reset password via swipe action
- **WHEN** boss swipes left on a staff member in the staff list
- **THEN** a "重置密码" button SHALL appear alongside the existing "移除" button

#### Scenario: Reset password dialog flow
- **WHEN** boss taps "重置密码" button
- **THEN** a dialog SHALL appear with a password input field and confirm/cancel buttons
- **AND** on confirm, the system SHALL call `PUT /api/staff/{id}/password` with the entered password
- **AND** on success, a toast SHALL show "密码重置成功"

#### Scenario: Reset password validation in dialog
- **WHEN** boss confirms the reset dialog with an empty password field
- **THEN** a toast SHALL show "请输入新密码" and the dialog SHALL remain open

### Requirement: API client method for reset password
The frontend API client SHALL provide a `resetStaffPassword` method.

#### Scenario: API client method signature
- **WHEN** calling `authApi.resetStaffPassword(staffId, newPassword)`
- **THEN** it SHALL send `PUT /api/staff/{staffId}/password` with body `{ "newPassword": newPassword }`

document.addEventListener('DOMContentLoaded', () => {
    console.log("auth.js script executing");

    // This script is for the signup page. If we're not on that page, do nothing.
    const signupButton = document.getElementById('signup-button');
    if (!signupButton) {
        return;
    }

    const emailInput = document.getElementById('email');
    const passwordInput = document.getElementById('password');
    const confirmPasswordInput = document.getElementById('confirm_password');
    const usernameInput = document.getElementById('username');

    const emailError = document.getElementById('email-error');
    const passwordError = document.getElementById('password-error');
    const confirmPasswordError = document.getElementById('confirm-password-error');
    const usernameError = document.getElementById('username-error');

    const validators = {
        email: (value) => {
            const emailRegex = /^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/;
            if (!value) return 'Email is required.';
            if (!emailRegex.test(value)) return 'Please enter a valid email.';
            return '';
        },
        password: (value) => {
            if (value.length < 8) return 'Password must be at least 8 characters long.';
            if (!/[a-z]/.test(value)) return 'Password must include a lowercase letter.';
            if (!/[A-Z]/.test(value)) return 'Password must include an uppercase letter.';
            if (!/\d/.test(value)) return 'Password must include a number.';
            return '';
        },
        confirm_password: (value) => {
            if (!value) return 'Please confirm your password.';
            if (value !== passwordInput.value) return 'The passwords do not match.';
            return '';
        },
        username: (value) => {
            if (value.length < 3) return 'Username must be at least 3 characters long.';
            return '';
        }
    };

    function validateAllFields() {
        const isEmailValid = validators.email(emailInput.value) === '';
        const isPasswordValid = validators.password(passwordInput.value) === '';
        const isConfirmPasswordValid = !confirmPasswordInput || validators.confirm_password(confirmPasswordInput.value) === '';
        const isUsernameValid = !usernameInput || validators.username(usernameInput.value) === '';


        signupButton.disabled = !(isEmailValid && isPasswordValid && isConfirmPasswordValid && isUsernameValid);
    }

    function setupLiveValidation(input, errorElement, validator) {
        input.addEventListener('input', () => {
            // Run the validator for the current field and display the message
            const errorMessage = validator(input.value);
            errorElement.textContent = errorMessage;

            // When the main password field changes, we must also re-validate the confirmation field
            if (input.id === 'password') {
                const confirmError = validators.confirm_password(confirmPasswordInput.value);
                confirmPasswordError.textContent = confirmError;
            }
            
            // Check the validity of all fields to enable/disable the submit button
            validateAllFields();
        });
    }

    setupLiveValidation(usernameInput, usernameError, validators.username);
    setupLiveValidation(emailInput, emailError, validators.email);
    setupLiveValidation(passwordInput, passwordError, validators.password);
    setupLiveValidation(confirmPasswordInput, confirmPasswordError, validators.confirm_password);

    // Run validation once on page load. This handles cases like browser autofill
    // or when the page is re-rendered by the server with pre-filled values.
    validateAllFields();
});

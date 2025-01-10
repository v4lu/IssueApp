import type { Config } from 'tailwindcss';

export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		container: {
			center: true,
			padding: {
				DEFAULT: '1.5rem'
			},
			screens: {
				'2xl': '80rem'
			}
		},
		extend: {
			colors: {
				border: 'hsl(var(--border))',
				input: 'hsl(var(--input))',
				ring: 'hsl(var(--ring))',
				background: {
					DEFAULT: 'hsl(var(--background))',
					muted: 'hsl(var(--background-muted))'
				},
				foreground: 'hsl(var(--foreground))',
				primary: {
					DEFAULT: 'hsl(var(--primary))',
					foreground: 'hsl(var(--primary-foreground))'
				},
				secondary: {
					DEFAULT: 'hsl(var(--secondary))',
					foreground: 'hsl(var(--secondary-foreground))'
				},
				destructive: {
					DEFAULT: 'hsl(var(--destructive))',
					foreground: 'hsl(var(--destructive-foreground))'
				},
				muted: {
					DEFAULT: 'hsl(var(--muted))',
					foreground: 'hsl(var(--muted-foreground))'
				},
				accent: {
					DEFAULT: 'hsl(var(--accent))',
					foreground: 'hsl(var(--accent-foreground))'
				},
				popover: {
					DEFAULT: 'hsl(var(--popover))',
					foreground: 'hsl(var(--popover-foreground))'
				},
				card: {
					DEFAULT: 'hsl(var(--card))',
					foreground: 'hsl(var(--card-foreground))'
				}
			},
			borderRadius: {
				xl: 'calc(var(--radius) + 4px)',
				lg: 'var(--radius)',
				md: 'calc(var(--radius) - 2px)',
				sm: 'calc(var(--radius) - 4px)'
			},
			perspective: {
				'1000': '1000px'
			},
			animation: {
				in: 'in 0.5s cubic-bezier(0.16, 1, 0.3, 1) forwards',
				float: 'float 10s ease-in-out infinite',
				pulse: 'pulse 10s ease-in-out infinite',
				ping: 'ping 1s cubic-bezier(0, 0, 0.2, 1) infinite'
			},
			keyframes: {
				'animate-in': {
					from: {
						opacity: '0',
						transform: 'translateY(10px)'
					},
					to: {
						opacity: '1',
						transform: 'translateY(0)'
					}
				},
				float: {
					'0%, 100%': {
						transform: 'translateY(0) scale(1)',
						opacity: '0.3'
					},
					'50%': {
						transform: 'translateY(-20px) scale(1.1)',
						opacity: '0.6'
					}
				},
				pulse: {
					'0%, 100%': {
						opacity: '1'
					},
					'50%': {
						opacity: '0.7'
					}
				}
			},
			transformOrigin: {
				center: 'center'
			},
			transitionTimingFunction: {
				'custom-ease': 'cubic-bezier(0.16, 1, 0.3, 1)'
			},
			delays: {
				'stagger-1': '100ms',
				'stagger-2': '200ms',
				'stagger-3': '300ms',
				'stagger-4': '400ms',
				'stagger-5': '500ms'
			}
		}
	},

	plugins: []
} satisfies Config;

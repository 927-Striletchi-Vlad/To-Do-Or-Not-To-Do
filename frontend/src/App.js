import * as React from 'react';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import IconButton from '@mui/material/IconButton';
import Typography from '@mui/material/Typography';
import PersonIcon from '@mui/icons-material/Person';
import Container from '@mui/material/Container';
import Tooltip from '@mui/material/Tooltip';
import TextField from '@mui/material/TextField';
import Drawer from '@mui/material/Drawer';

import PropTypes from 'prop-types';

import DialogCreateTodolist from './DialogCreateTodolist.js'
import TodolistsContainer from './TodolistsContainer.js'
import { createTheme, ThemeProvider } from '@mui/material/styles';
import './fonts/AbrilFatface-Regular.ttf';

export default function App() {
  // THEMES -------------------------------------
  const theme1 = createTheme({
  typography: {
    fontFamily: 'Abril',
  },
  components: {
    MuiCssBaseline: {
      styleOverrides: `
        @font-face {
          font-family: 'Abril';
          src: local("Abril"), url('./fonts/AbrilFatface-Regular.ttf') format('truetype');
          font-weight: normal;
          font-style: normal;
        }
      `,
    },
  },
});

  const [username, setUsername] = React.useState('Alice Smith');
  const [onTodoContainerChange, setOnTodoContainerChange] = React.useState(false);

  const [openCreateTodolist, setOpenCreateTodolist] = React.useState(false);

  const handleCloseCreateTodolist = () => {
    setOpenCreateTodolist(false);
  };

  const handleUsernameChange = event => {
      setUsername(event.target.value);
  };

  const [anchorElUser, setAnchorElUser] = React.useState(null);

  const handleOpenUserMenu = (event) => {
    setAnchorElUser(event.currentTarget);
  };

  const handleCloseUserMenu = () => {
    setAnchorElUser(null);
  };

  TodolistsContainer.propTypes = {
    userName: PropTypes.string.isRequired,
    onTodoContainerChange: PropTypes.bool.isRequired,
    setOnTodoContainerChange: PropTypes.func.isRequired,
  };

  DialogCreateTodolist.propTypes = {
    onClose: PropTypes.func.isRequired,
    open: PropTypes.bool.isRequired,
    userName: PropTypes.string.isRequired,
    setOnTodoContainerChange: PropTypes.func.isRequired,
  };

  return (
    <Box>
    <ThemeProvider theme={theme1}>
      <AppBar position="static" sx={{backgroundColor: '#5D4037'}} >
        <Container maxWidth="xl">
          <Toolbar disableGutters>
            <Typography
              variant="h6"
              noWrap
              component="a"
              href="/"
              sx={{
                mr: 2,
                display: { xs: 'none', md: 'flex' },
                fontWeight: 600,
                fontSize: '1.5rem',
                letterSpacing: '.3rem',
                color: 'inherit',
                textDecoration: 'none',
              }}
            >
              To do or not to do?
            </Typography>

            <Typography
              variant="h5"
              noWrap
              component="a"
              href=""
              sx={{
                mr: 2,
                display: { xs: 'flex', md: 'none' },
                flexGrow: 1,
                fontWeight: 600,
                fontSize: '1.5rem',
                letterSpacing: '.3rem',
                color: 'inherit',
                textDecoration: 'none',
              }}
            >
              To do or not to do?
            </Typography>
            <Drawer
              anchor="top" 
              open={Boolean(anchorElUser)}
              onClose={handleCloseUserMenu}
            >
              <Box 
                component="form"
                sx={{
                  '& > :not(style)': { m: 1, width: '25ch' },
                }}
              >
                <TextField
                  id="outlined-basic" 
                  label="Username" 
                  variant="outlined" 
                  value={username}
                  onChange={handleUsernameChange}
                />
                <TextField
                  id="outlined-basic" 
                  label="Password" 
                  variant="outlined" 
                  defaultValue="P@ssw0rd!"
                />
              </Box>
            </Drawer>
            <Box sx={{ flexGrow: 0 }}>
              <Tooltip title="Open settings">
                <IconButton onClick={handleOpenUserMenu} sx={{ p: 0, color: 'white' }}>
                  <PersonIcon />
                </IconButton>
              </Tooltip>
            </Box>
              <DialogCreateTodolist 
                onClose = {handleCloseCreateTodolist}
                open = {openCreateTodolist}
                userName={username} 
                setOnTodoContainerChange={setOnTodoContainerChange}
              />
            <Box sx={{ flexGrow: 0}}>
            </Box>
          </Toolbar>
        </Container>
      </AppBar>
    </ThemeProvider>
    <Box sx={{width: '100%', overflow: 'auto'}}>
      <TodolistsContainer 
        userName={username}
        onTodoContainerChange = {onTodoContainerChange}
        setOnTodoContainerChange = {setOnTodoContainerChange}
      />
    </Box>
  </Box>
  );
}

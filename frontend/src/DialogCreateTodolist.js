import * as React from 'react'; import Box from '@mui/material/Box';
import IconButton from '@mui/material/IconButton';
import DeleteIcon from '@mui/icons-material/Delete';
import AddCircleOutlineIcon from '@mui/icons-material/AddCircleOutline';
import Button from '@mui/material/Button';
import MenuItem from '@mui/material/MenuItem';
import TextField from '@mui/material/TextField';

import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import DialogTitle from '@mui/material/DialogTitle';
import Dialog from '@mui/material/Dialog';
import AddIcon from '@mui/icons-material/Add';
import PropTypes from 'prop-types';
import InputLabel from '@mui/material/InputLabel';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import Stack from '@mui/material/Stack';
import { styled } from '@mui/material/styles';

function CreateTodolist(title, priority, username, setOnTodoContainerChange) {
  // Note: the empty deps array [] means
  // this useEffect will run once
  // similar to componentDidMount()
    const body = JSON.stringify({ title: title, priority: priority});
    console.log(body);
    const address1 = process.env.REACT_APP_API_PREFIX + "/users/name/" + username;
    fetch(address1)
      .then(response => response.json())
      .then(data => {
        const requestOptions={
          method: "POST",
          body: body
        }
        const address2 = process.env.REACT_APP_API_PREFIX + "/todolists/" + data;
        fetch(address2, requestOptions)
          .then(res => res.json())
          .then(
            (result) => {
              setOnTodoContainerChange((prev) => !prev);
            },
            // Note: it's important to handle errors here
            // instead of a catch() block so that we don't swallow
            // exceptions from actual bugs in components.
            (error) => {
              alert("Error: " + error.message);
            }
          )
      }) 
  
}

function DialogCreateTodolistPopup(props) {
  const { onClose, open, userName, setOnTodoContainerChange} = props;
  const [titleInput, setTitleInput] = React.useState('');
  const [priorityInput, setPriorityInput] = React.useState('');


  const CreateTodolistTrigger = () => {
    CreateTodolist(titleInput, priorityInput, userName, setOnTodoContainerChange);
    onClose();
  }
  const handleClose = () => {
    onClose();
  };

  const CustomButton1 = styled(Button)({
    boxShadow: 'none',
    textTransform: 'none',
    fontSize: 16,
    padding: '6px 12px',
    border: '1px solid',
    lineHeight: 1.5,
    backgroundColor: '#fff',
    borderColor: '#FF5722',
    color: '#FF5722',
    '&:hover': {
      backgroundColor: '#D7CCC8',
      borderColor: '#FF5722',
      boxShadow: 'none',
    },
    '&:focus': {
      boxShadow: '0 0 0 0.2rem rgba(0,123,255,.5)',
    },
  });

  const CustomButton2 = styled(Button)({
    boxShadow: 'none',
    textTransform: 'none',
    fontSize: 16,
    padding: '6px 12px',
    border: '1px solid',
    lineHeight: 1.5,
    backgroundColor: '#FF5722',
    borderColor: '#FF5722',
    color: '#fff',
    '&:hover': {
      backgroundColor: '#FF5722',
      borderColor: '#FF5722',
      boxShadow: 'none',
    },
    '&:focus': {
      boxShadow: '0 0 0 0.2rem rgba(0,123,255,.5)',
    },
  });

  return (
    <Dialog onClose={handleClose} open={open}>
      <DialogTitle>Create Todolist</DialogTitle>
      <List >
          <ListItem sx={{padding: 2}} disableGutters>
            <TextField
              sx={{ width:"100%"}}
              value={titleInput}
              label="Title"
              onChange={(e) => setTitleInput(e.target.value)}
              variant="outlined" />
          </ListItem>
          <ListItem sx={{padding: 2}} disableGutters>
            <FormControl sx={{ width: "100%" }}>
              <InputLabel id="demo-simple-select-helper-label">Priority</InputLabel>
              <Select
                labelId="demo-simple-select-helper-label"
                id="priorityInput"
                value={priorityInput}
                label="Priority"
                onChange={(e) => setPriorityInput(e.target.value)}
              >
                <MenuItem value={10}>10</MenuItem>
                <MenuItem value={9}>9</MenuItem>
                <MenuItem value={8}>8</MenuItem>
                <MenuItem value={7}>7</MenuItem>
                <MenuItem value={6}>6</MenuItem>
                <MenuItem value={5}>5</MenuItem>
                <MenuItem value={4}>4</MenuItem>
                <MenuItem value={3}>3</MenuItem>
                <MenuItem value={2}>2</MenuItem>
                <MenuItem value={1}>1</MenuItem>
              </Select>
            </FormControl>
          </ListItem>
        

        <ListItem disableGutters>
          <Box
            padding={2}
            width="100%"
            justifyContent="center"
            alignItems="center"
          >
            <Stack direction="row" spacing={2}>
              <CustomButton1 variant="outlined" onClick={handleClose} startIcon={<DeleteIcon />}>
                Cancel
              </CustomButton1>
              <CustomButton2 variant="contained" onClick={CreateTodolistTrigger} endIcon={<AddIcon />}>
                Create
              </CustomButton2>
            </Stack>
          </Box>
        </ListItem>
      </List>
    </Dialog>
  );
}

DialogCreateTodolistPopup.propTypes = {
  onClose: PropTypes.func.isRequired,
  open: PropTypes.bool.isRequired,
  userName: PropTypes.string.isRequired,
  setOnTodoContainerChange: PropTypes.func.isRequired,
};

function DialogCreateTodolist(props) {
  const {userName, setOnTodoContainerChange} = props;
  const [open, setOpen] = React.useState(false);

  const handleClickOpen = () => {
    setOpen(true);
  };

  const handleClose = (value) => {
    setOpen(false);
  };
  

  return (
    <Box>
      <IconButton
        size="large"
        aria-label="account of current user"
        onClick={handleClickOpen}
        color="inherit"
      >
        <AddCircleOutlineIcon />
      </IconButton>
      <DialogCreateTodolistPopup
        open={open}
        onClose={handleClose}
        userName={userName}
        setOnTodoContainerChange={setOnTodoContainerChange}
      />
    </Box>
  );
}

export default DialogCreateTodolist;

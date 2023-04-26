import * as React from 'react';
import Box from '@mui/material/Box';
import PropTypes from 'prop-types';
import Skeleton from '@mui/material/Skeleton';
import Switch from '@mui/material/Switch';
import FormControlLabel from '@mui/material/FormControlLabel';


import Todolist from './Todolist';


export default function TodolistsContainer(props) {
  const {userName, onTodoContainerChange, setOnTodoContainerChange} = props
  const [error, setError] = React.useState(null);
  const [items, setItems] = React.useState(null);
  const [loaded, setLoaded] = React.useState(false);
  const [sorted, setSorted] = React.useState(false);
  const [sortedArray, setSortedArray] = React.useState(null);
  const [overwritten, setOverwritten] = React.useState(false);
  const [priorities, setPriorities] = React.useState(null);
  const [checkedSwitchPriority, setCheckedSwitchPriority] = React.useState(false);
  
  const handleSwitchChangePriority = (event) => {
    setCheckedSwitchPriority(event.target.checked);
  };

  // Note: the empty deps array [] means
  // this useEffect will run once
  // similar to componentDidMount()
  React.useEffect(() => {
    const address1 = process.env.REACT_APP_API_PREFIX + "/users/name/" + userName.toString();
    fetch(address1)
      .then(response => response.json())
      .then(data => {
        var address2="";
        if(checkedSwitchPriority){
          // address2 = process.env.REACT_APP_API_PREFIX + "/todolists/user/" + data + "/important";
          address2 = process.env.REACT_APP_API_PREFIX + "/todolists/user/" + data + "/importantwithpriority";
        }else{
          address2 = process.env.REACT_APP_API_PREFIX + "/todolists/user/" + data + "/withpriority";
        }
        fetch(address2)
          .then(res => res.json())
          .then(
            (result) => {
              setItems(result.data);
              setPriorities(result.priorities);
              setLoaded(true);
            },
            // Note: it's important to handle errors here
            // instead of a catch() block so that we don't swallow
            // exceptions from actual bugs in components.
            (error) => {
              setError(error);
            }
          )
      }) 
   
  }, [userName, onTodoContainerChange, checkedSwitchPriority]);
  
  Todolist.propTypes = {
    username: PropTypes.string,
    id: PropTypes.string,
    updatedParent: PropTypes.func,
  };

  React.useEffect(() => {
    //sort items by priority
    //make lst of tuples of item and priority
    // sort by priority
    if(!loaded){return;}

    var lst = [];
    for (var i = 0; i < items.length; i++) {
      lst.push([items[i], priorities[i]]);
    }
    lst.sort(function(a, b) {
      return b[1] - a[1];
    });
    var sorted = [];
    for (var i = 0; i < lst.length; i++) {
      sorted.push(lst[i][0]);
    }
    console.log(sorted);
    setSortedArray(sorted);
    setSorted(true);

  }, [loaded, items, priorities]);

  React.useEffect(() => {
    if(!sorted){return;}
    setItems(sortedArray);
    setOverwritten(true);
  }, [sorted]);

  if (error) {
    return <div>Error: {error.message}</div>;
  } else {
    if (overwritten){
      console.log(items)
      return (
        <>
          <Box sx={{margin: 2, color: '#D7CCC8'}}>
            <FormControlLabel
              control={
                <Switch color="warning" checked={checkedSwitchPriority} onChange={handleSwitchChangePriority} />
              }
              label="Priority > 5"
            />
          </Box>
          <>
            {items === null ? (
              <Skeleton>
                <Box sx={{display: 'flex', flexDirection: 'row', width: '100%', margin: 2}} />
              </Skeleton>
            ) : (
              <Box sx={{display: 'flex', flexDirection: 'row', width: '100%', margin: 2}}>
                {sortedArray && sortedArray.map(item => (
                  <Box sx={{padding: 2}}>
                    <Todolist key={item} sx={{display: 'inline-flex',}} 
                      username={userName} 
                      setUpdatedParent={setOnTodoContainerChange}
                      id={item}
                    />
                  </Box>
                ))}
              </Box>
            )
            }
          </>
        </>
      );
    }
  }
}

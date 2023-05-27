import { Component, OnInit } from '@angular/core';
import { UserService } from '../user.service';
import { ActivatedRoute, ParamMap } from '@angular/router';
import { RessourceInfo } from '../ressource-info';
import { FormGroup, FormControl } from '@angular/forms';
import { UserInfo } from '../user-info';
import { AuthenticationService } from 'src/app/login/authentication.service';
import { RessourceCredentialSsh } from '../ressource-credential-ssh';
import { RessourceCredentialWireguard } from '../ressource-credential-wireguard';
import { ReturnSshData } from '../return-ssh-data';
import { ReturnWireguardData } from '../return-wireguard-data';

@Component({
  selector: 'app-ressource-page',
  templateUrl: './ressource-page.component.html',
  styleUrls: ['./ressource-page.component.scss']
})
export class RessourcePageComponent implements OnInit {

  public bastion_id : string = '';
  public ressource_id : string = '';

  public ressource: RessourceInfo;

  public ajoutSSHForm: FormGroup;
  public pubkeySshCrtl: FormControl;

  public ajoutWireguardForm: FormGroup;
  public pubkeyWireguardCrtl: FormControl;

  public user!: UserInfo;

  public messageSSH: string = '';
  public messageWireguard: string = '';

  public ressourceCredentialSsh!: ReturnSshData;

  public ressourceCredentialWireguard!: ReturnWireguardData;

  public infoSsh='';
  public infoWireguard='';

  

  constructor(protected userService: UserService, private activRoute: ActivatedRoute, protected authenticationService: AuthenticationService) { 

    this.ressource = this.userService.get_ressource();

    this.user= this.authenticationService.get_info_login();

    this.pubkeySshCrtl = new FormControl('')


    this.ajoutSSHForm = new FormGroup({
        pubkeySSH: this.pubkeySshCrtl,
    })

    this.pubkeyWireguardCrtl = new FormControl('')

    this.ajoutWireguardForm = new FormGroup({
        pubkeyWireguard: this.pubkeyWireguardCrtl,
    })
  }

  ngOnInit(): void {

    this.ressource = this.userService.get_ressource();

    this.user= this.authenticationService.get_info_login();

    this.pubkeySshCrtl = new FormControl('')


    this.ajoutSSHForm = new FormGroup({
        pubkeySSH: this.pubkeySshCrtl,
    })

    this.pubkeyWireguardCrtl = new FormControl('')

    this.ajoutWireguardForm = new FormGroup({
        pubkeyWireguard: this.pubkeyWireguardCrtl,
    })

    this.activRoute.paramMap.subscribe((params: ParamMap) => {
      this.bastion_id = params.get('idBastion') || '';

      this.activRoute.paramMap.subscribe((params: ParamMap) => {
        this.ressource_id = params.get('idRessource') || '';

      });
    });
  }

  sendSSHKey(): void {

    this.messageSSH = '';
    this.infoSsh = '';  

    let pubkeySSH = this.pubkeySshCrtl.value.trim() as string;

    let creationSsh: RessourceCredentialSsh= {
      pubkey: pubkeySSH,
      username: this.user.mail
    }

    this.userService.generate_ssh_access(this.bastion_id, this.ressource_id, creationSsh).subscribe({

      next: (data) => {
        this.messageSSH = "Votre accès a bien été créé";

        this.ressourceCredentialSsh = data.data;

        this.infoSsh = `Cle publique du bastion : ${this.ressourceCredentialSsh.pubkey}}`
      },
      error: (err) => {
        this.messageSSH = "Impossible de créer votre accès";
      }
    });

  }

  sendWireguardKey(): void {

    this.messageWireguard = '';
    this.infoWireguard = '';

    let pubkeyWireguard = this.pubkeyWireguardCrtl.value.trim() as string;

    let creationWireguard: RessourceCredentialWireguard= {
      pubkey: pubkeyWireguard,
    }

    this.userService.generate_wireguard_access(this.bastion_id, this.ressource_id, creationWireguard).subscribe({

      next: (data) => {
        this.messageWireguard = "Votre accès a bien été créé";
        this.ressourceCredentialWireguard = data.data;
        this.infoWireguard = `Cle publique du bastion : ${this.ressourceCredentialWireguard.bastion_pubkey}\nPort : ${this.ressourceCredentialWireguard.port_wireguard}\n`
      },
      error: (err) => {
        this.messageWireguard = "Impossible de créer votre accès";
      }
    });

  }

  start_session() {

    this.userService.start_session(this.bastion_id, this.ressource_id).subscribe({

      next: (data) => {
        console.log(data);
      },
      error: (err) => {
        console.log(err);
      }
    });
  }

  stop_session() {
      
      this.userService.stop_session(this.bastion_id, this.ressource_id).subscribe({
  
        next: (data) => {
          console.log(data);
        },
        error: (err) => {
          console.log(err);
        }
      });
    }

}
